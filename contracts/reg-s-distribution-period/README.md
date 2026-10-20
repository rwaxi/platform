# reg-s-distribution-period

## What it does

Our compliance rule for Regulation S. While the distribution compliance period is active, it refuses any transfer or mint to a wallet whose identity has a U.S. country relation (code 840). After the period ends it allows everything. The decision and its parameters are in [ADR-007](../../docs/6-decisions/ADR-007-spike-compliance-rule-reg-s.md).

How it decides, per token:

- The period is active while the current ledger is lower than `period_end_ledger`. If no end was set, it is not active.
- By default only residence counts as a U.S. relation (`[Residence]`). For organizations, incorporation and operating jurisdiction count as residence.
- A recipient with no country data is refused while the period is active (fails closed).

It must be registered on the `CanTransfer` and `CanCreate` hooks of the compliance contract. `forced_transfer` and `recover_balance` do not go through those hooks, so the rule does not apply to them.

## Origin

Written by RWAXI. It implements OpenZeppelin's `ComplianceModule` trait from stellar-tokens 0.7.2.

## Public functions

| Function | Who can call | What it does |
|---|---|---|
| `set_period_end(token, period_end_ledger)` | owner | Sets the first ledger at which the restriction no longer applies. Emits `PeriodEndSet` |
| `set_restricted_relations(token, relations)` | owner | Sets which relations count as "U.S. person" (`Residence`, `Citizenship`, `TaxResidency`). Emits `RestrictedRelationsSet` |
| `set_identity_registry_storage(token, irs)` | owner | Sets where the module reads country data from |
| `set_compliance_address(compliance)` | owner | Changes the compliance contract the module belongs to |
| `can_transfer(from, to, amount, token)`, `can_create(to, amount, token)` | anyone (read-only) | The rule. False when the period is active and `to` is a U.S. person |
| `on_transfer`, `on_created`, `on_destroyed` | anyone | Do nothing; the rule keeps no state per operation |
| `period_end`, `is_active`, `restricted_relations`, `restricted_country`, `name`, `get_compliance_address` | anyone | Read state |

## Errors

| Code | Name | When |
|---|---|---|
| 410 | EmptyRelations | `set_restricted_relations` with an empty list |
| 397 | IdentityRegistryNotSet | The registry for that token was not set |

The token, not this module, returns 305 or 306 when the rule says no.

## Tests

13 unit tests in [`src/test.rs`](src/test.rs) and 5 end-to-end tests in [`tests/integration/src/spike.rs`](../../tests/integration/src/spike.rs). Line coverage of `lib.rs` is 100 %.

## Testnet

[`CDDOEBLE5NA65DIQIMRCJTHYDESMK5STN4WT3XLDAXQ3ENMMX6BVOULQ`](https://stellar.expert/explorer/testnet/contract/CDDOEBLE5NA65DIQIMRCJTHYDESMK5STN4WT3XLDAXQ3ENMMX6BVOULQ). Period end for the token: ledger 11517880. The one-year length is our reading of Rule 903; counsel has not confirmed it yet.
