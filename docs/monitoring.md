# On-chain monitoring

> Status: **v0.1** · 2026-10-15 · design. Today the events can be read by hand with the CLI (§4); the indexer that alerts on them is built in T2 ([ADR-005](6-decisions/ADR-005-event-indexer.md)).

Every rule of the platform lives in contracts on a public ledger, so the useful question is not "is the server up" but "did someone change a rule, a role or an identity". All of those changes emit events. This plan says which ones we watch, how loud each alert is, and what the person on call does.

## 1. What we watch

Severity: **P1** someone may be bypassing a rule or moving units by force, act now · **P2** a rule or role changed, confirm it was planned the same day · **P3** normal activity, review weekly.

| Event | Contract | Means | Severity |
|---|---|---|---|
| `ModuleRemoved` | compliance | A rule stopped running | P1 |
| `PeriodEndSet` | Reg S module | The Reg S period changed | P1 if earlier than before, P2 otherwise |
| `RestrictedRelationsSet` | Reg S module | What counts as a U.S. person changed | P1 |
| `CountryDataModified`, `CountryDataRemoved` | identity registry | An investor's country changed | P2; P1 if it moves from 840 to another code during the period |
| `RecoverySuccess`, `IdentityRecovered` | token, identity registry | Balance and identity moved to a new wallet | P2 |
| `Transfer` in a transaction that invoked `forced_transfer` | token | Units moved without the owner's signature. The event is the same as a normal transfer; the indexer tells them apart by the function the transaction invoked | P1 |
| `Paused`, `Unpaused` | token | All transfers stopped or resumed | P1 |
| `AddressFrozen`, `TokensFrozen`, `TokensUnfrozen` | token | An investor was blocked or released | P2 |
| `ComplianceSet`, `IdentityVerifierSet` | token | The token now trusts another contract | P1 |
| `RoleGranted`, `RoleRevoked`, `AdminTransferInitiated`, `AdminTransferCompleted` | all with access control | Someone gained or lost power | P1 |
| `TrustedIssuerAdded`, `TrustedIssuerRemoved`, `KeyAllowed`, `KeyRemoved` | claim topics and issuers, claim issuer | Who can certify KYC changed | P1 |
| `ClaimRevoked` | claim issuer | An investor lost KYC | P3 |
| `ModuleAdded` | compliance | A rule started running | P2 |
| `Mint`, `Burn` | token | Supply changed | P3; P2 if outside a subscription close |

Three checks that do not come from a single event:

- **Transfers rejected with 305.** They fail in simulation and never reach the ledger, so the chain shows nothing. T2: the API logs every rejected simulation and the indexer counts them per day; a spike means a broken integration or someone probing the rule.
- **Claims close to expiry.** Daily job: claims that expire in the next 30 days (threat T-D-5).
- **Reg S period against the calendar.** The period is set in ledgers and the law counts days ([ADR-007](6-decisions/ADR-007-spike-compliance-rule-reg-s.md)). Weekly job: estimate the date of `period_end_ledger` from the average close time of the last 7 days. P1 if it falls before the legal end date; the owner moves it forward with `set_period_end`.

## 2. How alerts reach people

| Tranche | Source | Delivery |
|---|---|---|
| Spike (now) | CLI query by hand (§4) | none |
| T2 | Indexer ([ADR-005](6-decisions/ADR-005-event-indexer.md)) reading events from Stellar RPC, stored with ledger and transaction hash | P1 to the on-call phone and the team channel at once; P2 to the channel; P3 in a weekly report |
| T3 (mainnet) | Same indexer, plus a second independent source (a public explorer API) compared daily | P1 also pages a second person |

The indexer is not authoritative. When an alert fires, the ledger is checked directly before acting.

## 3. What the on-call person does

| Alert | First step | Then |
|---|---|---|
| Rule removed or weakened (`ModuleRemoved`, `PeriodEndSet` earlier, `RestrictedRelationsSet`) | Find the transaction and the signer; ask the change owner | If it was not planned: pause the token, restore the rule from the record in `compliance-matrix.md`, rotate the key that signed |
| Forced transfer or recovery | Match it with a ticket (court order, lost device) | If there is no ticket: pause the token and freeze the receiving address |
| Role or issuer change | Match it with a planned change | If not planned: revoke the role or issuer, rotate keys |
| Country data change during the period | Check the KYC provider's decision for that investor | If it does not match: restore the data, freeze the address, review the backoffice access log |

## 4. Reading events today

Stellar RPC keeps events for a limited window (days, not months), which is why T2 adds an indexer. Until then:

```bash
TOKEN=$(jq -r .contracts.rwa_token deployments/testnet.json)
MODULE=$(jq -r .contracts.reg_s_distribution_period deployments/testnet.json)
START=$(jq -r .reg_s.deployed_at_ledger deployments/testnet.json)
stellar events --network testnet --start-ledger "$START" --id "$TOKEN" --id "$MODULE" --output pretty
```

---

← [Deployment guide](deploy.md) · [Index](README.md) · [7 · Model and diagrams](7-model/README.md) →
