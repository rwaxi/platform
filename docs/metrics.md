# On-chain metrics

> Status: **v0.1** · 2026-10-15 · definitions. Targets per tranche are set with the business plan and filled in before submission.

We measure growth only with numbers anyone can recompute from the public ledger. Each metric below says which contract events or views it comes from, so a reviewer does not need access to our database to check it.

## 1. Metrics

| # | Metric | Definition | Source on-chain | From |
|---|---|---|---|---|
| M1 | Verified investors | Wallets with an identity in the registry: `IdentityStored` minus `IdentityUnstored` | identity registry events | spike |
| M2 | Units issued | `total_supply` of each vehicle token (equals `Mint` minus `Burn`) | token view and events | spike |
| M3 | Holders | Wallets with a balance above zero, rebuilt from `Mint`, `Transfer` and `Burn` | token events | spike |
| M4 | Investor transfers | `Transfer` events between two investors, excluding mints, burns and forced transfers | token events | spike |
| M5 | USDC subscribed | USDC paid into the subscription contract per vehicle | subscription events | T2 |
| M6 | USDC distributed | Rental income claimed by investors | distribution events | T2 |
| M7 | Vehicles live | Tokens deployed by the vehicle factory | factory events | T2 |
| M8 | Compliance changes | Count of rule, role and identity changes (the P1 and P2 events of [monitoring.md](monitoring.md)) | all contracts | T2 |

M8 is not a growth number. We publish it because a platform that sells regulated securities should show how often its rules change, and who changed them.

Transfers rejected by a rule (error 305) do not reach the ledger, so they are not an on-chain metric. The API counts them for monitoring only.

## 2. How a third party checks them

- **Today (spike):** with the Stellar CLI or any RPC provider, reading the events of the contracts listed in [`deployments/testnet.json`](../deployments/testnet.json). Example in [monitoring.md §4](monitoring.md). Token views (`total_supply`, `balance`) can be read on Stellar Expert or in the [browser demo](https://rwaxi.github.io/platform/demo/).
- **T2:** the indexer publishes a read-only page with M1 to M8 per vehicle, and next to each number the ledger range it was computed from. Anyone can rerun the computation over the same range with the RPC and get the same result.
- **Retention.** Stellar RPC keeps events for a short window. The indexer stores every event with its ledger and transaction hash; the transaction itself stays on the ledger, so any number can still be traced back to its transactions.

## 3. Spike values (2026-10-15, testnet)

| Metric | Value |
|---|---|
| M1 Verified investors | 3 (Alice and Bob, Colombia; Carol, United States) |
| M2 Units issued | 100 ABF1 |
| M3 Holders | 2 (Alice 90, Bob 10) |
| M4 Investor transfers | 1 (Alice → Bob, 10 ABF1) |

These are test wallets created for the demo, not investors.

---

← [Test plan](test-plan.md) · [Index](README.md) · [Roadmap and verification](roadmap.md) →
