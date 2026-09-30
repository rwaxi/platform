# C4 Level 1 — System Context

```mermaid
flowchart LR
  classDef person fill:#08427b,stroke:#052e56,color:#fff
  classDef system fill:#1168bd,stroke:#0b4884,color:#fff
  classDef ext fill:#8a8a8a,stroke:#6b6b6b,color:#fff

  INV["<b>Investor</b><br/><i>[Person]</i><br/>Colombian or international.<br/>Passkey, KYC, USDC."]:::person
  OPS["<b>RWAXI operator</b><br/><i>[Person]</i><br/>Creates vehicles, approves<br/>investors, mints, pays out."]:::person
  LAW["<b>Securities counsel</b><br/><i>[Person]</i><br/>Defines transfer rules<br/>per vehicle."]:::person

  RWAXI["<b>RWAXI Platform</b><br/><i>[Software system]</i><br/>Issues and administers regulated<br/>RWA security tokens with<br/>on-chain compliance."]:::system
  ANC["<b>SEP-24 anchor</b><br/><i>[External]</i><br/>COP ↔ USDC"]:::ext

  KYC["<b>KYC/KYB provider</b><br/><i>[External]</i><br/>Documents, liveness, AML"]:::ext
  REL["<b>OZ Relayer · Channels</b><br/><i>[External]</i><br/>Submits tx, pays fees"]:::ext

  XLM["<b>Stellar network</b><br/><i>[External · Soroban]</i><br/>Token, identity and compliance<br/>contracts · USDC settlement"]:::ext
  EXP["<b>Stellar Expert</b><br/><i>[External]</i><br/>Public verification"]:::ext

  LAW -. "1 · rule set" .-> OPS
  INV -- "2 · onboard, subscribe, claim<br/>[app + passkey]" --> RWAXI
  OPS -- "3 · manage vehicles<br/>[backoffice]" --> RWAXI
  INV -. "4 · deposit / withdraw COP" .-> ANC
  RWAXI -- "5 · KYC sessions<br/>[HTTPS + webhook]" --> KYC
  RWAXI -- "6 · signed tx" --> REL
  REL -- "7 · submit" --> XLM
  RWAXI -- "8 · read state, events<br/>[Soroban RPC]" --> XLM
  ANC -- "9 · USDC" --> XLM
  XLM -. "10 · indexed by" .-> EXP
```

Legend: dark blue = people, blue = RWAXI, grey = external systems. Dotted lines are indirect or off-system relations.

## Actors

| Actor | Interacts through | Trust level |
|---|---|---|
| Investor | Mobile app + passkey on device | Untrusted input; every action is authorized by their smart account and checked by the token's compliance hooks |
| RWAXI operator | Backoffice (SSO + hardware key) | Privileged; mainnet actions via multisig |
| Securities counsel | Written rule set → operator configures modules | Source of truth for rules; never writes to the system directly |

## External systems

| System | Why it is external | Failure impact | Fallback |
|---|---|---|---|
| Stellar / Soroban RPC | Settlement layer | Platform read-only | Second RPC provider |
| KYC provider | Regulated identity verification | No new onboarding; existing holders unaffected | Provider abstraction in KYC service |
| OZ Relayer · Channels | Fee sponsorship | Investors cannot submit | Self-hosted OZ Relayer (same API) |
| SEP-24 anchor | Licensed fiat rails | No COP ramp; USDC deposits still work | Direct USDC deposit |
| Stellar Expert | Public verification | None on operations | Stellar Lab / RPC queries |

## What is deliberately **not** in the system

- No custody of investor keys: passkeys stay in the device secure enclave.
- No fiat handling: fiat enters and leaves only through a licensed anchor.
- No personal data on-chain.
