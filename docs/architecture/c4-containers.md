# C4 Level 2 — Containers

```mermaid
flowchart LR
  classDef person fill:#08427b,stroke:#052e56,color:#fff
  classDef cont fill:#438dd5,stroke:#2e6295,color:#fff
  classDef chain fill:#1168bd,stroke:#0b4884,color:#fff
  classDef db fill:#438dd5,stroke:#2e6295,color:#fff
  classDef ext fill:#8a8a8a,stroke:#6b6b6b,color:#fff

  INV["<b>Investor</b>"]:::person
  OPS["<b>RWAXI operator</b>"]:::person

  subgraph CLIENTS["Clients"]
    direction TB
    APP["<b>Investor app</b><br/><i>[React Native · Expo]</i><br/>Passkey, KYC, subscribe, claim"]:::cont
    BO["<b>Backoffice</b><br/><i>[Next.js]</i><br/>Vehicles, approvals, payouts"]:::cont
  end

  subgraph GCP["RWAXI off-chain · GCP"]
    direction TB
    API["<b>Platform API</b><br/><i>[NestJS · OpenAPI]</i><br/>Builds + simulates tx"]:::cont
    KYCS["<b>KYC & claim service</b><br/><i>[NestJS]</i><br/>Maps KYC → claims"]:::cont
    IDX["<b>Event indexer</b><br/><i>[Node worker]</i><br/>getEvents → DB"]:::cont
    DB[("<b>Platform DB</b><br/><i>[PostgreSQL]</i>")]:::db
    KMS["<b>KMS</b><br/><i>[GCP]</i><br/>Claim-issuer key"]:::cont
  end

  REL["<b>OZ Relayer · Channels</b><br/><i>[External]</i><br/>Pays fees"]:::ext
  PROV["<b>KYC provider</b><br/><i>[External]</i>"]:::ext
  ANC["<b>SEP-24 anchor</b><br/><i>[External]</i>"]:::ext

  subgraph CHAIN["Stellar · Soroban"]
    direction TB
    SA["<b>Smart account</b><br/><i>[OZ accounts]</i><br/>One per investor · passkey"]:::chain
    subgraph VEH["Per vehicle"]
      direction TB
      TOK["<b>rwa-token</b><br/><i>[OZ RWA]</i>"]:::chain
      CMP["<b>Compliance</b><br/><i>[OZ + custom modules]</i>"]:::chain
      SUB["<b>subscription</b><br/><i>[custom]</i>"]:::chain
      DIS["<b>distribution</b><br/><i>[custom]</i>"]:::chain
    end
    IDS["<b>Identity stack</b><br/><i>[OZ · shared]</i><br/>Registry, claims, verifier"]:::chain
    FAC["<b>vehicle-factory</b><br/><i>[custom · shared]</i>"]:::chain
    USDC["<b>USDC</b><br/><i>[Stellar Asset Contract]</i>"]:::chain
  end

  INV --> APP
  OPS --> BO
  APP -- "1 · HTTPS" --> API
  BO -- "2 · HTTPS" --> API
  API --- DB
  IDX --- DB
  KYCS -- "3 · sessions" --> PROV
  KYCS -- "4 · sign claim" --> KMS
  API -- "5 · signed tx" --> REL
  KYCS -- "6 · claim tx" --> REL
  REL -- "7 · submit" --> CHAIN
  IDX -. "8 · getEvents" .-> CHAIN
  APP -. "9 · passkey signature" .-> SA
  APP -. "10 · COP ramp" .-> ANC
  SA -- "11 · transfer · subscribe · claim" --> VEH
  TOK -- "12 · hooks" --> CMP
  VEH -- "13 · verify_identity<br/>country data" --> IDS
  VEH -- "14 · USDC" --> USDC
  FAC -. "15 · deploys" .-> VEH
```

## Container responsibilities

### On-chain

| Container | Owner code | Shared or per vehicle | Key rule |
|---|---|---|---|
| `rwa-token` | OZ `RWAToken` + thin RWAXI wrapper | per vehicle | Every transfer/mint/burn calls identity verifier and compliance; failure reverts atomically |
| Identity stack | OZ examples, configured | **shared** (one investor, one identity, many vehicles) | Only RWAXI's claim issuer is trusted at launch; topics: KYC, accreditation, AML |
| Compliance dispatcher | OZ `Compliance` | per vehicle | Modules registered per hook; config only by `module_operator` |
| Compliance modules | OZ + custom `reg-s-distribution-period` | module contracts shared, config per token | Policy modules exempt `Forced`/`Recovery` transfers; bookkeeping still runs |
| `subscription` | custom | per vehicle | Accepts USDC only from identity-verified accounts, within window and caps |
| `distribution` | custom | per vehicle | Pull model; entitlement = holdings at the distribution checkpoint |
| `vehicle-factory` | custom | shared | Deterministic salts; emits event with all addresses of the new set |
| Smart account | OZ `stellar-accounts` | per investor | Passkey (secp256r1) signer; RWAXI recovery signer limited by policy to `recover_balance` flow |

### Off-chain

| Container | Stateless? | Scaling | Notes |
|---|---|---|---|
| Platform API | yes | Cloud Run autoscale | Never holds investor keys; builds unsigned transactions and simulates them |
| KYC & claim service | yes | Cloud Run | Only component that can call the KMS signing key |
| Event indexer | cursor in DB | single worker + standby | Idempotent upserts by ledger + event index |
| PostgreSQL | — | Cloud SQL, PITR | No KYC documents stored |
| SDK | — | npm | Generated bindings (`stellar contract bindings typescript`) + helpers |

## Environments

| Env | Network | Purpose |
|---|---|---|
| `local` | Stellar quickstart container | Unit/integration tests in CI |
| `testnet` | Stellar testnet | Spike evidence, T1–T2 deliverables, public demo for reviewers |
| `mainnet` | Stellar pubnet | T3 launch |

Testnet is periodically reset by SDF; `scripts/deploy.sh` redeploys the full set from scratch and `deployments/testnet.json` keeps historical hashes and screenshots (see task 26 contingency plan).
