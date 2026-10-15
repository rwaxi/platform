# ADR-007 — Spike compliance rule: Reg S distribution compliance period

- Status: Proposed · 2026-09-24 · legal parameters to be confirmed by securities counsel (due 2026-10-09)

## Context
Before the SCF #46 submission (2026-11-04), the team must show verifiable evidence on Stellar testnet that it can build the core of the product: **a transfer rejected by an on-chain compliance rule, with a public transaction hash, plus an allowed transfer**. The spike is funded by RWAXI in 2026 and is not part of the award scope, so it must stay minimal: one token, one rule.

The rule chosen must be:
1. **Real** — taken from the actual offering structure of Avista Beach Tokenization Fund I, not a toy rule.
2. **Specific and verifiable** — a reviewer can understand why transfer A→B fails and A→C succeeds by looking at two hashes.
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

### Spike acceptance criteria (evidence for the submission)

| # | Evidence | Verified by |
|---|---|---|
| 1 | Public GitHub repo under the RWAXI organization, Apache-2.0 | URL |
| 2 | Unit tests: US recipient blocked during period; non-US allowed; US allowed after period; exact boundary ledger; forced/recovery exempt; unauthorized config rejected | CI green + coverage badge |
| 3 | Contract IDs on testnet (token, dispatcher, module, identity stack) | `deployments/testnet.json` + Stellar Expert |
| 4 | **Hash of A (CO) → B (US) transfer failing with `RegSRecipientUSPerson`** | Stellar Expert, no login |
| 5 | Hash of A (CO) → C (CO) transfer succeeding | Stellar Expert, no login |
| 6 | `scripts/demo-blocked-transfer.sh` reproduces both from a clean testnet | README |

## Consequences
- The spike directly demonstrates the Open Track asks: a Stellar-specific on-chain integration, creative use of the network's newest contract framework, and a team able to extend it.
- The module becomes a production component in T1 (hardening, edge cases, audit scope), so no spike work is thrown away, and the award budget covers only new work on it.
- **Legal caveat:** the spike demonstrates the mechanism. Parameters (period length, U.S.-person mapping) are placeholders until counsel confirms them; the README says so explicitly.
- **Technical risk — failed transaction hash:** Soroban RPC simulation will reject the A→B call, so a client stops before submitting. To obtain an on-chain failed transaction, the demo script must submit with explicitly set resources after a failed simulation. This must be validated in the first two days of the spike; fallback evidence is the simulation error plus the unit test, which is weaker.
