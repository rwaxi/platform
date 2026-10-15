# ADR-001 — Build on the OpenZeppelin `stellar-contracts` RWA module

- Status: Proposed · 2026-09-24

## Context
RWAXI needs a security token whose transfers are gated by investor identity and securities rules. SCF Build asks teams to "leverage existing ecosystem tools rather than building from scratch", and a custom token would carry unaudited core logic.

OpenZeppelin `stellar-contracts` (v0.7.2, Soroban SDK 26.1) ships an RWA module that ports T-REX to Soroban: `RWAToken` (mint, burn, forced transfer, recovery, freezes, pause, RBAC), an identity verification stack (identity registry storage with country data, claim topics and issuers, claim issuer, identity verifier) and a hook-based compliance dispatcher. The ready modules (`country_allow`, `country_restrict`, `initial_lockup_period`, `max_balance`, `supply_limit`, `time_transfers_limits`, `transfer_allow`) are not in 0.7.2: they ship in OZ `main` and the 0.8.0 release candidates (checked 2026-10-15).

## Options
1. Custom SEP-41 token with in-house compliance.
2. OZ RWA module, extended only where a rule is missing.
3. Stellar classic asset with `AUTH_REQUIRED` + clawback, compliance off-chain.

## Decision
Option 2. RWAXI writes custom code only for what OZ does not provide: the `reg-s-distribution-period` module (ADR-007), `subscription`, `distribution`, `balance-checkpoint`, `vehicle-factory`. Custom modules implement OZ's `ComplianceModule` trait so they plug into the same dispatcher.

## Consequences
- Core token and identity logic inherits OZ's review and audit process; RWAXI's audit scope (Audit Bank, after T2) is limited to custom contracts.
- Version is pinned; upgrades to OZ 0.8.x are an explicit, tested change.
- Option 3 is rejected because rules would live off-chain, which is exactly what the product is meant to avoid.
