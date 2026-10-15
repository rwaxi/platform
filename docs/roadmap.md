# Roadmap and verification by tranche

> Status: **v0.1** · 2026-10-15 · dates assume work starts on 2027-01-11. Costs per deliverable come from the budget (pending) and are added before submission.

What the award pays for, tranche by tranche, and how a reviewer checks each deliverable without asking us. Everything here is new work on top of the 2026 spike; nothing built before the award is billed ([overview §3](2-overview.md)).

Tranches follow the SCF Handbook: 10 % at award, then 20 % (MVP), 30 % (testnet) and 40 % (mainnet).

## T1 · MVP on testnet · weeks 1 to 4 · until 2027-02-07

| # | Deliverable | A reviewer verifies it by |
|---|---|---|
| T1.1 | Token with split roles (`admin`, `agent`, `claim_signer`, `module_operator`), pause, freeze and recovery | Contract ID on testnet; Stellar Expert shows a verified build of the `v1.x` release; tests for each role in CI |
| T1.2 | Identity stack with KYC, investor type and country topics, claim expiry and revocation | Contract IDs; tests that an expired and a revoked claim block transfers |
| T1.3 | Compliance rules for Fund I as defined by counsel ([compliance-matrix.md](compliance-matrix.md)), Reg S module hardened | Every row of the matrix has a contract, a parameter and a passing test linked from the matrix |
| T1.4 | KYC service that turns a provider decision into a signed claim, key in KMS | Testnet transaction of a claim issued by the service; architecture note on key access |
| T1.5 | Passkey smart account onboarding and a signed transfer from the app shell | Testnet transaction signed by a passkey smart account; short screen recording |
| T1.6 | Fee sponsorship through OZ Relayer | Testnet transaction whose fee was paid by the relayer account |

Evidence package: release tag, `deployments/testnet.json`, CI run with coverage of at least 80 % of lines, and a demo script that reproduces T1.1 to T1.6.

## T2 · Full product on testnet · weeks 5 to 8 · until 2027-03-07

| # | Deliverable | A reviewer verifies it by |
|---|---|---|
| T2.1 | Subscription contract: USDC in, batch mint at close or refund | Contract ID; testnet close with mint and refund transactions |
| T2.2 | Distribution contract: USDC deposit and pro-rata claim | Contract ID; deposit and claim transactions |
| T2.3 | Vehicle factory: one set of contracts per fund | Factory ID and the IDs of a second vehicle created by it |
| T2.4 | Event indexer and the public metrics page ([metrics.md](metrics.md)) | URL; the numbers match a recount over the stated ledger range |
| T2.5 | Backoffice for vehicles, claims and deposits, with two-person approval on privileged actions | Screen recording; testnet transactions it produced |
| T2.6 | Threat model and monitoring live, as the Handbook requires for T2 | [threat-model.md](threat-model.md) updated for T1–T2 code; alerts firing on the events in [monitoring.md](monitoring.md), shown with a test alert |

Evidence package as in T1, plus the request to the SDF Audit Bank with the threat model and the test plan.

## T3 · Mainnet · weeks 9 to 12 · until 2027-04-04

| # | Deliverable | A reviewer verifies it by |
|---|---|---|
| T3.1 | Invariant tests and the audit package; findings from the Audit Bank addressed | Report link; commits that close each finding |
| T3.2 | Mainnet deployment from an attested release, admin and agent behind multisig | Mainnet contract IDs; Stellar Expert shows verified builds and the multisig signers |
| T3.3 | First issuance of Fund I with real investors under counsel's rules | Mainnet mint and subscription transactions |
| T3.4 | TypeScript SDK published on npm under Apache-2.0 | npm package URL and version |
| T3.5 | Runbooks and live alerts | [monitoring.md](monitoring.md) runbooks; alert history for the first issuance |

If SCF limits the final tranche to testnet for jurisdiction reasons, T3.2 and T3.3 move to a testnet rehearsal with the same release, and the mainnet launch follows outside the award.

## What is not in any tranche

Institutional website, marketing, legal work, audits (requested through the Audit Bank), referral program and IP registration. RWAXI pays for those separately.

---

← [On-chain metrics](metrics.md) · [Index](README.md) · [7 · Model and diagrams](7-model/README.md) →
