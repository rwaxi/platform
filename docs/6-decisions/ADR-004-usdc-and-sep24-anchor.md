# ADR-004 — USDC as settlement asset; COP only through a SEP-24 anchor

- Status: Proposed · 2026-09-24 · depends on task 38 decision

## Context
Subscriptions and distributions need a dollar asset; Colombian investors hold COP. RWAXI must not handle fiat.

## Options
1. Native USDC (Circle) on Stellar + SEP-24 anchor for COP.
2. Bridged or wrapped stablecoins.
3. Custom COP token.

## Decision
Option 1. The anchor is integrated behind an interface (`ramp` module) and **no tranche deliverable depends on a third-party anchor being live**: testnet flows use USDC directly.

## Consequences
- USDC SAC is the only asset contract the custom contracts accept.
- Anchor choice can change without touching contracts.
- If no COP anchor is confirmed, the submission describes the ramp as an interface, not as a funded deliverable.
