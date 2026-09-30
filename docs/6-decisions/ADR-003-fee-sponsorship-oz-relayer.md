# ADR-003 — Fee sponsorship through OpenZeppelin Relayer + Channels

- Status: Proposed · 2026-09-24

## Context
Investors must never need to acquire XLM. Launchtube, the previous SDF sponsored-submission service, is archived and marked legacy; its README directs users to the OpenZeppelin Relayer with the Channels plugin.

## Options
1. Investor buys XLM.
2. Launchtube (legacy).
3. OpenZeppelin Relayer + Channels (managed service), self-hosted OZ Relayer as fallback.
4. Fully custom fee-bump service.

## Decision
Option 3.

## Consequences
- Backend holds relayer credentials in Secret Manager; the app never does.
- Budget line for relayer operation in T2/T3; rate limits and per-investor caps defined in the monitoring plan.
- Self-hosted fallback uses the same API, so switching is configuration, not code.
