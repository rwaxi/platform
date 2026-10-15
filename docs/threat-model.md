# Threat model (STRIDE)

> Status: **v0.1** · 2026-10-15 · covers the testnet spike in depth and the target platform at design level. Follows the four questions of Stellar's [threat-modeling guide](https://developers.stellar.org/docs/build/security-docs/threat-modeling/threat-modeling-how-to). Input for the SDF Audit Bank and for [monitoring.md](monitoring.md).

## 1. What are we working on?

The system, its data flows, data stores and the six trust boundaries (TB1 to TB6) are in [5 · Security](5-security.md). The spike is the on-chain part of TB5: the token, the compliance dispatcher with the Reg S module, and the identity stack (registry, verifier, claim topics and issuers, claim issuer, one identity contract per investor). Deployment and keys are in [deploy.md](deploy.md).

Assets we protect, in order:

1. **The rule holds.** No unit reaches a U.S. person during the distribution compliance period, and no unit reaches a wallet without valid identity.
2. **Balances.** Nobody moves units they do not own, except the operator through forced transfer or recovery.
3. **Identity data.** Nothing on-chain identifies a person by name or document.
4. **Availability.** Verified investors can transfer when the rules allow it.

## 2. What can go wrong?

IDs are `T-<letter>-<n>`. "Spike" means the threat applies to what is deployed on testnet today.

### Spoofing

| ID | Threat | Where | Spike |
|---|---|---|---|
| T-S-1 | Someone gets the claim-signing key and issues KYC claims for wallets that never passed KYC | DS3, claim issuer | yes |
| T-S-2 | Someone gets the admin or manager key and acts as the operator | DS10, every contract | yes |
| T-S-3 | A wallet presents a claim signed for another identity or another network | claim issuer | yes |
| T-S-4 | A fake token or module is passed off as RWAXI's | TB5, explorers | yes |

### Tampering

| ID | Threat | Where | Spike |
|---|---|---|---|
| T-T-1 | The manager changes a U.S. investor's country data to Colombia, and the Reg S module lets units through | identity registry | yes |
| T-T-2 | The module owner moves `period_end_ledger` back and lifts the restriction early | Reg S module | yes |
| T-T-3 | The manager removes the Reg S module from the `CanTransfer` hook and the rule disappears without a code change | compliance dispatcher | yes |
| T-T-4 | The module owner narrows `restricted_relations` (for example, drops residence) | Reg S module | yes |
| T-T-5 | A dependency or build step changes the WASM that is deployed | supply chain | yes |

### Repudiation

| ID | Threat | Where | Spike |
|---|---|---|---|
| T-R-1 | An operator denies having changed a rule or forced a transfer | all admin functions | yes |
| T-R-2 | An off-chain decision (KYC approval) cannot be traced to the claim it produced | KYC service, DS2 | no (T1) |

### Information disclosure

| ID | Threat | Where | Spike |
|---|---|---|---|
| T-I-1 | Country codes and holdings on a public ledger are linked to a person | DS5, DS6 | yes |
| T-I-2 | Claim data or URIs leak personal data | DS7 | yes |
| T-I-3 | Spike keys or the claim-signing key end up in the repository | repo, CI | yes |

### Denial of service

| ID | Threat | Where | Spike |
|---|---|---|---|
| T-D-1 | A verified investor with no country data is refused (the module fails closed) | Reg S module | yes |
| T-D-2 | Contract storage expires (TTL) and calls fail until it is restored | all contracts | yes |
| T-D-3 | Testnet is reset and the evidence disappears | testnet | yes |
| T-D-4 | The operator pauses the token or freezes an address without cause | token | yes |
| T-D-5 | Claims expire for many investors at once | claim issuer | yes |

### Elevation of privilege

| ID | Threat | Where | Spike |
|---|---|---|---|
| T-E-1 | Forced transfer and recovery skip the compliance modules, so the operator can deliver units to a U.S. person during the period | token | yes |
| T-E-2 | Anyone binds or rebinds the module's compliance address | Reg S module | yes |
| T-E-3 | A non-owner configures the module | Reg S module | yes |
| T-E-4 | One key holds every role in every contract | all | partly |

## 3. What are we going to do about it?

Status: **done** (in the spike, with a test or a check), **planned** (tranche given), **accepted** (by design, with a reason).

| Threat | Response | Status |
|---|---|---|
| T-S-1 | Testnet: key in `.secrets/`, out of git (checked over the full history before the repo went public). T1: key in cloud KMS, only the KYC service identity can request a signature, every signature logged. Claims expire; the issuer can revoke a claim and remove a key | done (testnet) · planned T1 |
| T-S-2 | Testnet: separate `admin` and `manager` keys, testnet only. T3: both behind a Stellar multisig account; signing on hardware keys | partly done · planned T3 |
| T-S-3 | The signed message includes the network id, the issuer address, the identity, the topic and a nonce (OpenZeppelin `build_claim_message`); a claim for another network or identity fails verification. We hit this on testnet: a claim signed with the wrong network id was rejected | done |
| T-S-4 | Every contract is deployed from a release whose WASM hash is attested (SEP-55); Stellar Expert shows each one as a verified build of `rwaxi/platform`. Contract IDs are published in `deployments/testnet.json` and the README | done |
| T-T-1 | Country data changes emit `CountryDataModified`; [monitoring.md](monitoring.md) alerts on every one. T2: country data is written only by the KYC service from the provider's decision, not by a person; backoffice edits need a second approver | monitored (T2) · planned T2 |
| T-T-2 | `set_period_end` emits `PeriodEndSet` and is alerted on. T1: only allow moving the end later, never earlier, unless a multisig approves; decide with counsel whether the end can ever be shortened | planned T1 |
| T-T-3 | `ModuleRemoved` is alerted on. T3: dispatcher manager behind multisig. A test checks that the token rejects with 305 while the module is registered | monitored (T2) · planned T3 |
| T-T-4 | `set_restricted_relations` emits `RestrictedRelationsSet` and rejects an empty list (test `empty_relation_list_is_rejected`). Values come from [compliance-matrix.md](compliance-matrix.md) | done · monitored (T2) |
| T-T-5 | Exact versions pinned (`=0.7.2`, `=26.1.1`, Rust 1.98.1), `Cargo.lock` committed, release built in GitHub Actions and attested; deploy script checks the attestation before deploying | done |
| T-R-1 | Every admin function emits an event; the ledger is the audit log. T2: the indexer stores them with ledger and transaction hash | done (events) · planned T2 |
| T-R-2 | T1: the KYC service stores the provider reference next to the claim id | planned T1 |
| T-I-1 | Only numeric country codes and relation types go on-chain, no names or documents (design rule in 5 · Security). Counsel to confirm this is acceptable under Colombia's Law 1581 and U.S. rules | done · waiting for counsel |
| T-I-2 | Claim data in the spike is a fixed tag and an expiry; the URI points to a placeholder. T1: URIs never carry personal data | done |
| T-I-3 | `.secrets/` and `.stellar/` are in `.gitignore`; history scanned before publishing. T1: GitHub secret scanning on | done · planned T1 |
| T-D-1 | Accepted: refusing an investor whose residence we cannot read is safer than letting a possible U.S. person in. The KYC flow writes country data before the claim, so a verified investor always has it. Test `recipient_without_country_data_fails_closed` | accepted |
| T-D-2 | Storage TTL is extended on every read and write (30 days for module entries, OpenZeppelin defaults elsewhere). An expired entry makes the call fail, it does not skip the rule, so expiry is an availability problem, not a bypass. T2: the indexer extends TTL of entries that are about to expire | partly done · planned T2 |
| T-D-3 | `scripts/deploy-testnet.sh` redeploys everything from the release in about three minutes; `redeploy-after-reset.sh` (C-08) wraps it | done · C-08 in progress |
| T-D-4 | `Paused`, `AddressFrozen` and `TokensFrozen` are alerted on. T3: pause behind multisig, with a runbook | monitored (T2) · planned T3 |
| T-D-5 | T2: the indexer warns 30 days before a claim expires | planned T2 |
| T-E-1 | Accepted: forced transfer and recovery exist for court orders and lost devices, and OpenZeppelin's token does not run compliance modules on them. Test `forced_transfer_is_exempt_from_the_period` documents it; the recovery case is listed in [test-plan.md](test-plan.md) §6. Every forced transfer and `RecoverySuccess` is alerted on; T3 puts the agent role behind multisig | accepted · monitored (T2) |
| T-E-2 | The compliance address is set once in the constructor; a second set fails (test `compliance_address_is_one_time`) | done |
| T-E-3 | Every setter calls `require_auth` on the owner (test `config_requires_the_owner`) | done |
| T-E-4 | T1: split roles as in the overview (`admin`, `agent`, `claim_signer`, `module_operator`); T3: admin and agent behind multisig | planned T1, T3 |

## 4. Did we do a good job?

- The Reg S rule, its boundary ledger, the owner checks and the exemptions are covered by 18 tests that run in CI with a line-coverage floor of 80 % ([test-plan.md](test-plan.md)).
- The same scenario runs on testnet from attested release artifacts and can be reproduced by anyone in the [browser demo](https://rwaxi.github.io/platform/demo/) or with `scripts/demo-blocked-transfer.sh`.
- Open items that need people, not code: counsel on T-I-1 and on whether T-T-2 may ever shorten the period; a second reviewer for every admin change once the team grows.
- Before mainnet (T3): this model is reviewed again with the T1 and T2 code, and the package goes to the SDF Audit Bank.

---

← [5 · Security](5-security.md) · [Index](README.md) · [Deployment guide](deploy.md) →
