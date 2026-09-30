# ADR-005 — Own event indexer over Soroban RPC

- Status: Proposed · 2026-09-24

## Context
Portfolio views, backoffice reports, on-chain growth metrics (an Open Track requirement) and the T2 monitoring plan all need queryable history of token, identity, compliance and distribution events.

## Options
1. Query RPC / explorer on demand.
2. Third-party indexer (e.g., Mercury).
3. Own worker on `getEvents` → PostgreSQL.

## Decision
Option 3, with Stellar Expert as the public verification surface.

## Consequences
- Idempotent ingestion keyed by ledger + event index; cursor persisted.
- The same event stream drives alerts defined in the monitoring plan (pause, forced transfer, module changes, claim revocations, failed-compliance rate).
- RPC retention is limited, so the indexer must run continuously from the first testnet deploy of T1.
