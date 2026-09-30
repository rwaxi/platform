# Data Flow Diagram and Trust Boundaries

> Status: **Draft v0.1** · 2026-09-24
> Purpose: answer *"What are we working on?"* in the Stellar threat-modeling process ([How-To Guide](https://developers.stellar.org/docs/build/security-docs/threat-modeling/threat-modeling-how-to)). This is the input to the STRIDE threat model (`threat-model.md`) and to the Tranche 2 monitoring plan (`monitoring.md`), and it covers the "Dataflow Diagram" item of the Audit Bank readiness checklist.

Notation, as defined by the How-To Guide:

| Element | Meaning | Shape in the diagram |
|---|---|---|
| **External entity** | Outside RWAXI's control | grey rectangle |
| **Process** | Code or systems RWAXI controls | blue rounded box (orange = RWAXI custom contract) |
| **Data store** | Where data is stored, including Soroban contract storage | cylinder |
| **Data flow** | Data moving between elements | numbered arrow `Dn` |
| **Trust boundary** | Group where trust assumptions change | red dashed box `TBn` |

---

## 1. System DFD (full platform, target architecture)

Split in two views so the flows stay readable. Flow IDs are shared between both views and the tables in §4.

### 1a. Off-chain view (TB1–TB4, TB6)

```mermaid
flowchart LR
  classDef ext fill:#8a8a8a,stroke:#6b6b6b,color:#fff
  classDef proc fill:#1168bd,stroke:#0b4884,color:#fff
  classDef store fill:#f4f4f4,stroke:#555,color:#222

  subgraph TB1["TB1 · Investor device"]
    INV["Investor"]:::ext
    APP("Investor app"):::proc
    PK[("DS1 · Passkey key<br/>secure enclave")]:::store
  end

  subgraph TB2["TB2 · Operator workstation"]
    OPS["RWAXI operator"]:::ext
    BO("Backoffice"):::proc
    AGK[("DS10 · Agent / admin signer<br/>hardware key · multisig")]:::store
  end

  subgraph TB3["TB3 · RWAXI cloud (GCP)"]
    API("Platform API"):::proc
    KYCS("KYC & claim service"):::proc
    IDX("Event indexer"):::proc
    DB[("DS2 · PostgreSQL")]:::store
    SM[("DS4 · Secret Manager")]:::store
    subgraph TB4["TB4 · KMS"]
      KMS[("DS3 · Claim-issuer key")]:::store
    end
  end

  subgraph TB6["TB6 · Third-party services"]
    PROV["KYC provider"]:::ext
    PDOCS[("DS9 · ID documents")]:::store
    REL["OZ Relayer · Channels"]:::ext
    ANC["SEP-24 anchor"]:::ext
  end

  XLM["TB5 · Stellar network<br/>(detail in §2)"]:::ext

  INV -- "D1" --> APP
  APP -- "D2 · WebAuthn" --> PK
  APP -- "D3 · requests, signed auth" --> API
  APP -- "D4 · documents, selfie" --> PROV
  PROV --- PDOCS
  PROV -- "D5 · signed webhook" --> KYCS
  KYCS -- "D6 · sign claim" --> KMS
  KYCS -- "D7" --> DB
  API -- "D8" --> DB
  API --- SM
  API -- "D10 · signed tx" --> REL
  KYCS -- "D10 · claim tx" --> REL
  REL -- "D11 · submit" --> XLM
  API -- "D9 · simulate" --> XLM
  XLM -. "D17 · events" .-> IDX
  IDX -- "D18" --> DB
  OPS -- "D19 · SSO" --> BO
  BO -- "D20 · admin requests" --> API
  BO -- "D21 · tx to sign" --> AGK
  APP -. "D22 · SEP-24 · PII" .-> ANC
  ANC -. "D23 · USDC" .-> XLM

  style TB1 fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
  style TB2 fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
  style TB3 fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
  style TB4 fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
  style TB6 fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
```

Image: [`architecture/diagrams/dfd-1a-offchain.png`](architecture/diagrams/dfd-1a-offchain.png)

### 1b. On-chain view (TB5)

```mermaid
flowchart LR
  classDef ext fill:#8a8a8a,stroke:#6b6b6b,color:#fff
  classDef proc fill:#1168bd,stroke:#0b4884,color:#fff
  classDef custom fill:#e8833a,stroke:#b5621f,color:#fff
  classDef store fill:#f4f4f4,stroke:#555,color:#222

  REL["OZ Relayer<br/>(TB6)"]:::ext
  ANC["SEP-24 anchor<br/>(TB6)"]:::ext

  subgraph TB5["TB5 · Stellar network · public ledger"]
    SA("Smart account"):::proc
    SUBD("subscription /<br/>distribution"):::custom
    TOK("rwa-token"):::proc
    DS5[("DS5 · Balances,<br/>frozen, allowances")]:::store
    IDV("Identity verifier"):::proc
    DS6[("DS6 · Identity registry<br/>wallet → identity, country")]:::store
    DS7[("DS7 · Signed claims")]:::store
    CMP("Compliance dispatcher"):::proc
    REGS("reg-s module +<br/>OZ modules"):::custom
    DS8[("DS8 · Module config<br/>and state")]:::store
  end

  IDX["Event indexer<br/>(TB3)"]:::ext
  PUB["Public · Stellar Expert"]:::ext

  REL -- "D11 · submit" --> SA
  REL -- "D11 · claim tx" --> IDV
  ANC -. "D23 · USDC" .-> SA
  SA -- "D12 · transfer" --> TOK
  SA -- "D12 · subscribe, claim" --> SUBD
  SUBD -- "D16 · mint" --> TOK
  TOK --- DS5
  TOK -- "D13 · verify_identity" --> IDV
  IDV --- DS6
  IDV --- DS7
  TOK -- "D14 · hooks" --> CMP
  CMP --> REGS
  REGS --- DS8
  REGS -- "D15 · country" --> DS6
  TOK -. "D17 · events" .-> IDX
  DS5 -. "D24 · public" .-> PUB

  style TB5 fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
```

Image: [`architecture/diagrams/dfd-1b-onchain.png`](architecture/diagrams/dfd-1b-onchain.png)

---

## 2. Spike DFD (testnet evidence, October 2026)

The spike has no app, backend or third parties. A developer operates it with the Stellar CLI.

```mermaid
flowchart LR
  classDef ext fill:#8a8a8a,stroke:#6b6b6b,color:#fff
  classDef proc fill:#1168bd,stroke:#0b4884,color:#fff
  classDef custom fill:#e8833a,stroke:#b5621f,color:#fff
  classDef store fill:#f4f4f4,stroke:#555,color:#222

  subgraph TBA["TB-A · Developer machine / CI runner"]
    DEV["Developer"]:::ext
    CLI("Stellar CLI ·<br/>demo script"):::proc
    KEYS[("Testnet keys<br/>admin, issuer, A, B, C")]:::store
  end

  subgraph TBB["TB-B · Stellar testnet · public"]
    TOK("rwa-token"):::proc
    IDV("Identity verifier"):::proc
    CMP("Compliance dispatcher"):::proc
    REGS("reg-s-distribution-period"):::custom
    BAL[("Balances")]:::store
    IRS[("Identity registry<br/>A=CO · B=US · C=CO")]:::store
    CFG[("Module config<br/>period_end_ledger")]:::store
  end

  EXP["Stellar Expert"]:::ext

  DEV --> CLI
  CLI -- "S1 · sign" --> KEYS
  CLI -- "S2 · deploy, configure, register identities" --> TOK
  CLI -- "S3 · transfer A→B, A→C" --> TOK
  TOK --- BAL
  TOK -- "S4 · verify_identity" --> IDV
  IDV --- IRS
  TOK -- "S5 · on_transfer" --> CMP
  CMP --> REGS
  REGS -- "S6 · country of recipient" --> IRS
  REGS --- CFG
  TBB -. "S7 · tx hashes, contract IDs" .-> EXP

  style TBA fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
  style TBB fill:none,stroke:#c0392b,stroke-width:2px,stroke-dasharray:6 4
```

Image: [`architecture/diagrams/dfd-2-spike.png`](architecture/diagrams/dfd-2-spike.png)

---

## 3. Trust boundaries

| ID | Boundary | What is trusted inside | Trust assumption crossing it |
|---|---|---|---|
| TB1 | Investor device | App code as shipped; the OS secure enclave | Nothing from the device is trusted by the backend or contracts without a passkey signature. The private key never leaves the enclave |
| TB2 | Operator workstation | The operator, authenticated via SSO + hardware key | Privileged on-chain actions need the agent/admin signer (multisig on mainnet); a backoffice session alone cannot move tokens |
| TB3 | RWAXI cloud | API, KYC service, indexer, DB | Requests from TB1/TB2 are untrusted input. The API is **not** in the trust path of compliance: anyone can call the contracts directly, so every rule is enforced on-chain |
| TB4 | KMS | Claim-issuer key; only the KYC service identity can request signatures | Compromise of this key = ability to issue false identity claims. Highest-value asset off-chain |
| TB5 | Stellar network | Contract code as deployed (verifiable via SEP-55 build attestation) and ledger state | Any account can invoke any contract function; authorization comes only from `require_auth` and RBAC. **Everything in TB5 is public** |
| TB6 | Third-party services | Each provider for its own function | KYC webhooks are authenticated and re-checked; the relayer can delay or censor but cannot forge signatures; the anchor handles its own PII and fiat |

---

## 4. Data flows

| ID | From → To | Data | Crosses | Protection |
|---|---|---|---|---|
| D1 | Investor → App | User input, biometric gesture | — | Biometric verified by the OS, never sent |
| D2 | App ↔ Passkey | WebAuthn challenge / signature | — (inside TB1) | Key non-exportable |
| D3 | App → API | Session token, requests, signed auth entries | TB1 → TB3 | TLS 1.2+, short-lived session bound to the passkey |
| D4 | App → KYC provider | ID documents, selfie (**PII**) | TB1 → TB6 | Provider SDK; RWAXI never receives the images |
| D5 | KYC provider → KYC service | Decision, country, investor type | TB6 → TB3 | Signed webhook, verified; decision re-fetched from provider API |
| D6 | KYC service ↔ KMS | Claim digest / signature | TB3 → TB4 | IAM: only the KYC service identity; every signature logged |
| D7 | KYC service → DB | KYC status, provider reference | — | No documents stored |
| D8 | API ↔ DB | Profiles, vehicles, indexed state | — | Private network, least-privilege DB roles |
| D9 | API → Soroban RPC | Transaction simulation | TB3 → TB5 | Read-only |
| D10 | API / KYC service → Relayer | Signed transaction XDR | TB3 → TB6 | Credentials in Secret Manager; payload already signed |
| D11 | Relayer → Stellar | Submission with fee bump | TB6 → TB5 | Signatures inside the transaction cannot be altered |
| D12 | Smart account → contracts | `transfer`, `subscribe`, `claim` | inside TB5 | `require_auth` via passkey signature |
| D13 | Token → Identity verifier | `verify_identity(from)`, `verify_identity(to)` | inside TB5 | Reverts if a required claim is missing, expired or revoked |
| D14 | Token → Compliance | Hooks with pre-operation `AccountSnapshot` | inside TB5 | Runs in the same transaction; a panic reverts everything |
| D15 | Reg S module → Identity registry | Country relations of the recipient | inside TB5 | Read-only |
| D16 | Subscription → Token | Mint at offering close | inside TB5 | Contract holds the agent role for minting only |
| D17 | Stellar → Indexer | Contract events | TB5 → TB3 | Public data; the indexer is **not authoritative**, the ledger is |
| D18 | Indexer → DB | Normalized state | — | Idempotent by ledger + event index |
| D19 | Operator → Backoffice | SSO session | inside TB2 | SSO + hardware key |
| D20 | Backoffice → API | Admin requests | TB2 → TB3 | Role checks in the API; audit log |
| D21 | Backoffice → Agent signer | Privileged transaction to sign | inside TB2 | Human review of the transaction before signing; multisig on mainnet |
| D22 | App → Anchor | SEP-24 interactive flow (**PII to anchor**) | TB1 → TB6 | Anchor's own KYC; RWAXI stores only the transaction reference |
| D23 | Anchor → Smart account | USDC | TB6 → TB5 | Standard SAC transfer |
| D24 | Ledger → Public | All on-chain state and events | TB5 → public | By design; see §5 on what is on-chain |

---

## 5. Data stores and classification

| ID | Store | Location | Data | Classification |
|---|---|---|---|---|
| DS1 | Passkey private key | Investor device, secure enclave | Signing key | **Secret** · never leaves the device |
| DS2 | PostgreSQL | TB3 | Name, email, KYC status, provider reference, indexed on-chain state | **Personal data** · encrypted at rest, backups with PITR |
| DS3 | Claim-issuer key | TB4 (KMS) | Ed25519 signing key | **Secret · critical** |
| DS4 | Secret Manager | TB3 | Relayer and provider API credentials | **Secret** |
| DS5 | Token storage | TB5 (Soroban) | Balances, frozen amounts, allowances | **Public · pseudonymous** |
| DS6 | Identity registry storage | TB5 (Soroban) | Wallet → identity contract, numeric ISO-3166 country codes and relation type | **Public · pseudonymous** — no names, no document numbers |
| DS7 | Identity claims | TB5 (Soroban) | Claim topic, issuer, signature, expiry | **Public · pseudonymous** |
| DS8 | Compliance module state | TB5 (Soroban) | Period end ledger, restricted relations, lock entries | **Public** |
| DS9 | Identity documents | TB6 (KYC provider) | Document images, liveness data | **Sensitive personal data** · governed by the provider agreement |
| DS10 | Agent / admin signer | TB2 (hardware key; multisig on mainnet) | Privileged signing keys | **Secret · critical** |

**Design rule:** no data that identifies a person by name or document ever reaches TB5. Linking a wallet to a person requires DS2 or DS9, both outside the public ledger.

---

## 6. Assumptions to validate

1. The KYC provider supports a client-side SDK, so documents go straight from device to provider (D4). If not, the flow changes and DS2 would temporarily hold documents.
2. Country codes on-chain (DS6) are acceptable to counsel from a privacy standpoint for the target jurisdictions (Colombia Law 1581, U.S.).
3. The relayer managed service (D10–D11) accepts signed Soroban invocations with smart-account auth entries.
4. The spike keys (TB-A) are testnet-only and are never reused on mainnet.
