# C4 Level 1: System Context

![System context](diagrams/c4-1-context.png)

Three people sit around the platform: the investor, the RWAXI operator and the securities counsel. Counsel never touches the system. They hand the operator a written rule set, and the operator turns it into module configuration on the token.

Anything that moves money or ownership ends on Stellar. The platform prepares the transaction, the OZ Relayer submits it and pays the fee, and the result is public in Stellar Expert, where a reviewer can check it without asking us. KYC and the COP ramp stay with licensed providers on purpose. We don't want to hold identity documents or fiat.

Colors: yellow for people, blue for what we build or configure, orange for contracts we write ourselves, grey for systems we don't control. The picture is exported from the Archi model in [`archi/rwaxi-architecture.archimate`](archi/rwaxi-architecture.archimate).

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
