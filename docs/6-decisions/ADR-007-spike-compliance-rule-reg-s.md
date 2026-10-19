# ADR-007 — Spike compliance rule: Reg S distribution compliance period

- Status: Accepted for the spike · 2026-10-19 · parameters below are our reading of Regulation S; securities counsel has not confirmed them yet
- History: Proposed 2026-09-24 · 2026-10-19 investors named as in the deployment (Alice and Bob in Colombia, Carol in the U.S.); the first version had B and C swapped

## Context
Before the SCF #46 submission (2026-11-04), the team must show verifiable evidence on Stellar testnet that it can build the core of the product: **a transfer rejected by an on-chain compliance rule, with a public transaction hash, plus an allowed transfer**. The spike is funded by RWAXI in 2026 and is not part of the award scope, so it must stay minimal: one token, one rule.

The rule chosen must be:
1. **Real** — taken from the actual offering structure of Avista Beach Tokenization Fund I, not a toy rule.
2. **Specific and verifiable** — a reviewer can understand why transfer Alice→Carol fails and Alice→Bob succeeds by looking at the error of one and the hash of the other.
3. **Not already shipped by OpenZeppelin** — so the spike proves the team can extend the framework, not just configure it.
4. **Small** — buildable in ~10 developer days together with token and identity setup.

Offering context: the fund sells to non-US investors (Colombia) under Regulation S. During the **distribution compliance period** (Rule 903), securities sold offshore may not be offered or sold to a U.S. person. After the period ends, the restriction lifts. For equity of a non-reporting U.S. issuer (Category 3) the period is generally one year; the exact length and definitions are counsel's call.

**Scope assumption (2026-10-06):** Fund I is sold only under Regulation S. There is no Regulation D tranche in the same token, so this rule does not block any investor the fund can legally sell to. If a Reg D offering is added later on the same token, this ADR must be reopened, because the rule would also block U.S. accredited investors.

## Options

| # | Option | Real | Verifiable | Beyond OZ | Small |
|---|---|---|---|---|---|
| 1 | OZ `country_restrict` with US blocked; admin removes it at period end | yes | yes | **no** — pure configuration | yes |
| 2 | OZ `initial_lockup_period` | partly (holding period ≠ Reg S) | yes | **no** | yes |
| 3 | OZ `max_balance` / `supply_limit` | weak | yes | **no** | yes |
| 4 | **Custom `reg-s-distribution-period` module** | **yes** | **yes** | **yes** | **yes** |
| 5 | Full modular engine (all rules) | yes | yes | yes | **no** — this is award scope (T1) |

Option 1 depends on an operator remembering to act on a date; the rule's time bound is not enforced by the chain. Option 5 would be building award deliverables before the award, which SCF does not reimburse.

## Decision
Option 4: a custom compliance module implementing OpenZeppelin's `ComplianceModule` trait, registered in the OZ compliance dispatcher for the **`CanTransfer` and `CanCreate`** hooks. OZ's `on_transfer`/`on_created` are post-action notifications; the blocking hooks are the read-only `can_transfer`/`can_create`, which return `bool`.

### Rule
For a given token, while `current_ledger < period_end_ledger`, reject any **Standard** or **Delegated** transfer, and any mint, whose recipient wallet has a **U.S. country relation** in the identity registry storage. `Forced` and `Recovery` transfers are exempt (privileged operations, per OZ guidance). After `period_end_ledger`, the module allows everything.

### Interface (sketch)

```rust
pub trait RegSDistributionPeriod: ComplianceModule {
    // all three require auth from the module owner (set in the constructor)
    fn set_identity_registry_storage(e: &Env, token: Address, irs: Address);
    fn set_period_end(e: &Env, token: Address, period_end_ledger: u32);
    fn set_restricted_relations(e: &Env, token: Address, relations: Vec<RelationKind>); // default: [Residence]
    fn period_end(e: &Env, token: Address) -> u32;
    fn is_active(e: &Env, token: Address) -> bool;
}
// can_transfer / can_create: read recipient CountryData from IRS;
// return false if active and any restricted relation == 840.
// A recipient with no country data is also refused while the period is active (fail closed).
```

- Time is measured in **ledger sequence**, like OZ's lockup module, not timestamps.
- Which country relations count as "U.S. person" (residence only, or also tax residency) is a parameter, set from counsel's mapping.
- Events: `period_end_set`, `restricted_relations_set`.

### Parameters

Counsel has not sent the rules yet, so we set the parameters from the text of Regulation S. We use these values on testnet and change them when counsel answers. None of them needs a code change: the owner sets all three on the deployed module.

Assumptions. Fund I is issued by a U.S. entity that does not file reports under the Exchange Act, and its units are equity securities. That puts it in Category 3. If counsel says the issuer is foreign or the units are debt, the category changes and so does the period.

| Parameter | Value we use | Where it comes from |
|---|---|---|
| Length of the period | One year | Rule 903(b)(3)(iii)(A): one year for equity of a non-reporting issuer, six months if the issuer reports |
| When the period starts | When the distribution is complete, not at the first sale | Rule 902(f). In a continuous offering the period starts when the distributor certifies the distribution is complete. While Fund I keeps selling, the owner moves `period_end_ledger` forward |
| Who is a U.S. person | Residence only. Default `restricted_relations = [Residence]` | Rule 902(k). For a natural person the test is residence. Citizenship and tax residency are not the test: a U.S. citizen living in Colombia is not a U.S. person under Reg S. Blocking on citizenship would refuse investors the fund can sell to |
| Recipient with no country data | Refused while the period is active | Rule 903(b)(3)(iii)(B)(1): the buyer must certify that it is not a U.S. person. No record means no certification |
| Transfers between non-U.S. holders during the period | Allowed | Rule 904 allows offshore resales to non-U.S. persons during the period |
| After the period | The module allows everything | Rule 905 makes these units restricted securities. A resale into the U.S. still needs an exemption. For a holder that is not an affiliate, Rule 144 lifts the restriction after one year with a non-reporting issuer, the same length as the period. Affiliates are a separate rule (compliance matrix R-09) |

How we turn one year into ledgers. The deploy script uses `17280 * 365` ledgers, which assumes 5 seconds per ledger. Ledgers on Stellar have closed a little slower than that, so the period ends after the legal date and not before. That is the safe side. If close times drop below 5 seconds, the period would end early, so the monitoring plan watches the gap between `period_end_ledger` and the legal end date, and the owner corrects it.

The module does what Rule 903(b)(3)(iii)(B)(4) asks of the issuer: refuse to register any transfer that does not follow Regulation S. Here the refusal happens on chain.

### Spike acceptance criteria (evidence for the submission)

| # | Evidence | Verified by |
|---|---|---|
| 1 | Public GitHub repo under the RWAXI organization, Apache-2.0 | URL |
| 2 | Unit tests: US recipient blocked during period; non-US allowed; US allowed after period; exact boundary ledger; forced/recovery exempt; unauthorized config rejected | CI green, coverage in the job summary |
| 3 | Contract IDs on testnet (token, dispatcher, module, identity stack), built from an attested release | `deployments/testnet.json` + Stellar Expert (`verified`) |
| 4 | Alice (CO) → Carol (US) transfer refused with `TransferNotCompliant` (error 305) | `scripts/demo-blocked-transfer.sh` and the public demo. The refusal happens in simulation, so there is no hash (see the last point below) |
| 5 | Hash of Alice (CO) → Bob (CO) transfer succeeding | Stellar Expert, no login |
| 6 | `scripts/demo-blocked-transfer.sh` reproduces both from a clean testnet | README |

## Consequences
- The spike directly demonstrates the Open Track asks: a Stellar-specific on-chain integration, creative use of the network's newest contract framework, and a team able to extend it.
- The module becomes a production component in T1 (hardening, edge cases, audit scope), so no spike work is thrown away, and the award budget covers only new work on it.
- **Legal caveat:** the spike demonstrates the mechanism. The parameters come from our reading of Rules 902 to 905, listed above with their source. Counsel confirms or corrects them; until then they are working values, and the README says so.
- **Failed transaction hash (resolved 2026-10-15):** the RPC simulation rejects the Alice→Carol call with error 305, so nothing reaches the ledger. We chose not to force the submission of a transaction we know fails just to get a hash. The evidence is the reproducible error from the script and the demo, plus the integration tests that assert 305 on transfer and 306 on mint.
