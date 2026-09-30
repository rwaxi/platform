# C4 Level 2: Containers

![Containers](diagrams/c4-2-containers.png)

The view has three blocks. On the left, the two clients: the investor app (Expo) and the backoffice (Next.js). In the middle, what runs in our GCP project: API, KYC and claim service, event indexer, Postgres and KMS. At the bottom, the contracts on Soroban.

Some decisions are easier to see here than in the ADRs:

- The investor signs with a passkey on the phone (flow 9). The API builds and simulates transactions, but it never holds an investor key.
- Only the KYC and claim service can use the claim-signing key in KMS (flow 4). A compromised API still can't issue identities.
- Everything goes out through the OZ Relayer (flows 5, 6 and 7), so the investor never needs to buy XLM.
- The indexer reads contract events from RPC (flow 8) and writes them to Postgres. That database feeds screens and reports. The ledger is the source of truth.

Inside the Stellar box we only list the contracts. How they call each other is in [C4 Level 3](c4-components.md). Source model: [`archi/rwaxi-architecture.archimate`](archi/rwaxi-architecture.archimate).

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
