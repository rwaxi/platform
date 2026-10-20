# Test plan

> Status: **v0.1** · 2026-10-15 · covers the testnet spike (`v0.1`). Sections marked *planned* belong to the award (T1 to T3) and are not built yet.

What we test, at which level, how a reviewer re-runs it, and what is still missing. The rule under test is the Reg S distribution compliance period from [ADR-007](6-decisions/ADR-007-spike-compliance-rule-reg-s.md).

## 1. Levels

| Level | Where | Runs on | What it proves |
|---|---|---|---|
| Unit | `contracts/reg-s-distribution-period/src/test.rs` | `cargo test`, CI | The module's logic alone, with a stub identity registry |
| End to end | `tests/integration/src/spike.rs` | `cargo test`, CI | The real contracts together: token, compliance dispatcher, identity registry, verifier, claim issuer with Ed25519 claims, and the module |
| Testnet | `scripts/deploy-testnet.sh`, `scripts/demo-blocked-transfer.sh` | Stellar testnet, by hand | The same scenario on the network, with contract IDs and a transaction hash anyone can check |

The end-to-end tests do not mock anything RWAXI deploys. They register the same contracts that go to testnet, sign KYC claims with a real Ed25519 key and read errors from the token.

## 2. The scenario

Three verified investors, each with an identity contract holding a signed KYC claim and a residence in the identity registry:

| Investor | Residence | During the period | After the period |
|---|---|---|---|
| Alice | Colombia (170) | can send and receive | can send and receive |
| Bob | Colombia (170) | can receive | can receive |
| Carol | United States (840) | **cannot receive** (transfer or mint) | can receive |

## 3. ADR-007 acceptance criteria → tests

| # | Criterion (ADR-007) | Test | Level |
|---|---|---|---|
| 1 | U.S. recipient blocked during the period | `us_recipient_is_blocked_during_period` · `alice_to_bob_allowed_and_alice_to_carol_blocked_during_period` (asserts error 305) · `minting_to_a_us_investor_is_blocked_during_period` (asserts error 306) | unit · e2e |
| 2 | Non-U.S. recipient allowed | `non_us_recipient_is_allowed_during_period` · `alice_to_bob_…` | unit · e2e |
| 3 | U.S. recipient allowed after the period | `us_recipient_is_allowed_after_period` · `us_investor_can_receive_after_the_period_ends` | unit · e2e |
| 4 | Exact boundary ledger | `boundary_ledger_is_exact` (last blocked ledger is `period_end_ledger - 1`; `period_end_ledger` itself is free) · the e2e test checks both ledgers on the token | unit · e2e |
| 5 | Forced and recovery transfers exempt | `forced_transfer_is_exempt_from_the_period` | e2e |
| 6 | Unauthorized configuration rejected | `config_requires_the_owner` (no auth, and auth from another address) · `compliance_address_is_one_time` | unit |

Behaviour that is not in ADR-007's list but is part of the rule as built:

| Behaviour | Test |
|---|---|
| No restriction until a period is configured | `nothing_is_blocked_until_a_period_is_set` |
| Recipient with no country data is refused during the period (fails closed) | `recipient_without_country_data_fails_closed` |
| Only the configured relations count (residence by default; citizenship once enabled) | `only_configured_relations_count` |
| A U.S.-incorporated organization counts as a U.S. person | `us_organization_is_blocked_by_incorporation` |
| An empty list of restricted relations is rejected | `empty_relation_list_is_rejected` |
| Views report the configuration; notification hooks change nothing | `views_report_the_configuration` · `notification_hooks_do_nothing` |
| A wallet without an identity cannot receive (identity stack, not the module) | `a_wallet_without_identity_cannot_receive` |
| An expired KYC claim stops transfers to that wallet (error 304, from the identity verifier) | `expired_kyc_claim_blocks_transfers` |

Recovery transfers are exempt for the same reason forced transfers are: OpenZeppelin's token does not call `can_transfer` on them. The spike tests the forced case only; the recovery case is added in T1 (§6).

## 4. How to run it

```bash
cargo test --workspace                       # 18 tests: 13 unit, 5 end to end
cargo llvm-cov --workspace \
  --ignore-filename-regex '(/test\.rs|tests/integration|tools/)' \
  --summary-only                             # coverage, same filter as CI
./scripts/deploy-testnet.sh                  # deploy from scratch (needs funded rwaxi-* testnet keys)
./scripts/demo-blocked-transfer.sh           # Alice -> Bob succeeds, Alice -> Carol fails with #305
```

On testnet, the blocked transfer fails when the network simulates it. The CLI does not submit a transaction that fails simulation, so the rejection leaves no hash on the ledger. The evidence is the contract error `#305` and the module's `can_transfer → false` in the diagnostic events, which the demo script prints and anyone can reproduce.

## 5. Quality gates (CI)

Every push to `main` and every pull request runs [`ci.yml`](../.github/workflows/ci.yml):

| Gate | Fails when |
|---|---|
| `cargo fmt --check` | code is not formatted |
| `cargo clippy --workspace --all-targets -D warnings` | any lint warning |
| tests with `cargo-llvm-cov` | a test fails, or line coverage of the workspace drops below 80 % |
| `stellar contract build` | any contract does not build to WASM |

Coverage on 2026-10-15: 85.6 % of lines across the contracts; `reg-s-distribution-period/src/lib.rs` at 100 %. Test files, the integration crate and `tools/` are excluded from the measure. The contracts adapted from OpenZeppelin's examples count towards the 80 % but are only tested through the end-to-end scenario; their library code has its own tests upstream.

Releases (`v*` tags) build each contract in GitHub Actions and attest its hash ([SEP-55](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0055.md)). A testnet deployment counts as evidence only if it was made from those release artifacts; `deployments/testnet.json` records which (`wasm_source`).

## 6. Not covered yet

| Gap | Why it matters | When |
|---|---|---|
| Recovery transfer during the period | Exempt by design; only forced transfers are tested | T1 |
| Roles as in the overview (`admin`, `agent`, `claim_signer`, `module_operator`) | The spike uses OpenZeppelin's example roles (`admin`, `manager`) | T1 |
| Claim revocation reaching the token | A revoked KYC claim must stop transfers. Expiry is covered by `expired_kyc_claim_blocks_transfers` | T1 |
| Property-based tests over ledgers, amounts and country sets | Unit tests cover chosen points, not the space between them | T1 |
| Integration on the Stellar quickstart container | Today the end-to-end tests run in the Soroban test environment, not on a local network | T2 |
| Testnet smoke test in CI after every release | The demo is run by hand | T2 |
| Invariants for the audit package (no balance reaches a U.S. person during the period, supply conservation) | Input for the SDF Audit Bank | T3 |

## 7. Waiting on counsel

These are parameters, not code. Tests already cover both settings of each; what changes is the configuration deployed.

| Question | Current setting | Test that covers the alternative |
|---|---|---|
| Which relations make a U.S. person: residence only, or also citizenship and tax residency | residence only | `only_configured_relations_count` |
| Length of the distribution compliance period (Rule 903) | ~1 year (6,307,200 ledgers), placeholder | `boundary_ledger_is_exact` |
| Whether a recipient with no country data is refused | refused (fails closed) | `recipient_without_country_data_fails_closed` |

---

← [6 · Decisions](6-decisions/README.md) · [Index](README.md) · [Compliance matrix](compliance-matrix.md) →
