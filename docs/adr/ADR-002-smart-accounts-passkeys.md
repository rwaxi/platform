# ADR-002 — Investor wallets are smart accounts with passkeys

- Status: Proposed · 2026-09-24

## Context
Target investors are not crypto-native. Seed phrases are the main cause of abandonment and loss. The wallet address must also be stable enough to be registered in the identity registry.

As of September 2026: `kalepail/passkey-kit` is archived and continues as `stellar/passkey-kit`, whose README states its wallet contract is **unaudited** and points to `stellar/smart-account-kit`, built on the **audited** OpenZeppelin `stellar-accounts` smart account (context rules, policies, WebAuthn verifier).

## Options
1. Classic Stellar accounts with seed phrases (Freighter, etc.).
2. `stellar/passkey-kit` wallet contract.
3. OpenZeppelin `stellar-accounts` smart account + `stellar/smart-account-kit` SDK.

## Decision
Option 3 for retail investors. Classic wallets (option 1) remain possible for institutional investors in a later phase; they go through the same identity and compliance checks.

## Consequences
- Passkey (secp256r1 / WebAuthn) as primary signer; a RWAXI recovery signer restricted by policy to the recovery flow.
- Same vendor (OZ) for token and account contracts reduces integration and audit surface.
- Risk: smart-account-kit is younger than passkey-kit. Mitigation: T1 deliverable includes an end-to-end signed transfer test; fallback to passkey-kit is a client-side swap.
