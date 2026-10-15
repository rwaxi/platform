# Compliance matrix

> Status: **v0.1 skeleton** · 2026-10-15 · one rule implemented (Reg S distribution period). The other rules for Avista Beach Tokenization Fund I come from securities counsel and **have not been received yet** (due 2026-10-09). We do not invent rules: every empty cell below waits for counsel.

Each row takes one legal requirement to the place where the chain enforces it: the contract or module, the parameter that encodes it, and the test that proves it. A rule is "done" only when all four columns are filled and the test runs in CI.

## 1. How a rule is enforced

A rule can live in one of three places. Which one is a design choice per rule, recorded in the last column of §2.

| Place | Enforced by | Good for |
|---|---|---|
| **Identity** | Signed claims in the investor's identity contract, checked by the identity verifier on every transfer and mint (`verify_identity`) | Who may hold the token at all: KYC passed, investor type, claim expiry |
| **Compliance module** | A contract registered on the compliance dispatcher. `CanTransfer` / `CanCreate` hooks can reject; `Transferred` / `Created` / `Destroyed` hooks keep the module's own state | Rules about the transfer itself: country of the recipient, dates, balances, supply, volumes |
| **Off-chain** | The backend or the offering documents, before anything reaches the chain | Rules the chain cannot see: suitability questionnaires, source of funds documents, minimum ticket in the subscription flow |

Forced and recovery transfers skip the compliance modules (the token does not call `can_transfer` on them) but recovery still verifies the identity of the new wallet.

## 2. Matrix — Avista Beach Tokenization Fund I

| # | Requirement (counsel) | Source | Where | Contract / module | Parameter | Test | Status |
|---|---|---|---|---|---|---|---|
| R-01 | During the distribution compliance period, no offer or sale to a U.S. person | Reg S, Rule 903 (Category 3) | Module | `reg-s-distribution-period` (ours, [ADR-007](6-decisions/ADR-007-spike-compliance-rule-reg-s.md)) | `period_end_ledger`; `restricted_relations` (default: residence) | `test-plan.md` §3, criteria 1–6 | **Built, on testnet.** Period length and relations are placeholders until counsel confirms |
| R-02 | Every holder has passed KYC with an approved provider | _counsel_ | Identity | claim topic `1` (KYC), trusted `claim-issuer` | topic id, issuer, signing key, claim validity | `a_wallet_without_identity_cannot_receive` | **Built** with one test issuer and key. Provider (task 41) and validity period pending |
| R-03 | AML / sanctions screening of every holder | _counsel_ | Identity, maybe module | _pending_: second claim topic (AML) and/or `country_restrict` with sanctioned jurisdictions | _pending_ | — | Waiting for counsel |
| R-04 | Countries where the fund may be sold, and where it may not | _counsel_ | Module | `country_allow` or `country_restrict` (OZ) | ISO 3166-1 numeric codes | — | Waiting for counsel |
| R-05 | Investor type allowed to hold (e.g. non-U.S. person, professional or retail) | _counsel_ | Identity | claim topic per investor type | topic ids | — | Waiting for counsel |
| R-06 | Minimum and maximum investment per investor | _counsel_ | Off-chain (minimum) · Module (maximum) | subscription flow (T2) · `max_balance` (OZ) | amounts in token units | — | Waiting for counsel |
| R-07 | Holding or lock-up period after subscription | _counsel_ | Module | `initial_lockup_period` (OZ) | lock-up length in ledgers | — | Waiting for counsel |
| R-08 | Maximum size of the offering | Offering memorandum | Module | `supply_limit` (OZ) | cap in token units | — | Waiting for the memorandum |
| R-09 | Who may receive a secondary transfer after the period (transferability) | _counsel_ | Module | `transfer_allow` (OZ) or a custom module | _pending_ | — | Waiting for counsel |
| R-10 | Limits on transfer volume per period | _counsel_ | Module | `time_transfers_limits` (OZ) | window and limit | — | Only if counsel requires it |
| R-11 | Maximum number of holders | _counsel_ | Module | none in OZ; custom module (T1) | holder cap | — | Only if counsel requires it |
| R-12 | Recovery of a lost wallet | _counsel_ (approve the procedure) | Identity + token | `recover_identity` (registry) then `recover_balance` (token), operator role | — | _planned_ T1 (`test-plan.md` §6) | Procedure needs JP's legal approval |

The rows R-03 to R-11 are the categories RWAXI asked counsel to define (countries, investor type, KYC, amounts, lock-up, transferability). The mechanism in each row is the candidate, not a decision: counsel's answer can move a rule to another place or remove it.

## 3. The OpenZeppelin modules we expect to use

The modules named above are not in `stellar-tokens` 0.7.2, the stable version this repository pins. They ship in OpenZeppelin's `main` branch and the `v0.8.0` release candidates (ports of the T-REX modules). For each one we register, we either move to 0.8.0 when it leaves release candidate or adapt the module into `contracts/`, as we did with the identity contracts.

| Module | Blocks through | Keeps state on | Typical parameter |
|---|---|---|---|
| `country_allow` / `country_restrict` | `CanTransfer`, `CanCreate` | — | list of country codes |
| `initial_lockup_period` | `CanTransfer` | `Created`, `Transferred`, `Destroyed` | lock-up length |
| `max_balance` | `CanTransfer`, `CanCreate` | `Created`, `Transferred`, `Destroyed` | max balance per identity |
| `supply_limit` | `CanCreate` | `Created`, `Destroyed` | total supply cap |
| `time_transfers_limits` | `CanTransfer` | `Transferred` | window and limit per identity |
| `transfer_allow` | `CanTransfer` | — | allowed addresses |
| `reg-s-distribution-period` (ours) | `CanTransfer`, `CanCreate` | — | period end ledger, restricted relations |

A module that keeps state must be registered on its state hooks as well as its blocking hooks, or its counters drift; OpenZeppelin's modules check this on first use (`verify_required_hooks`).

## 4. What we need from counsel

For each of R-03 to R-11: the rule in one sentence, its legal source, and the value of its parameter. Plus, for R-01: the exact length of the distribution compliance period and which relations (residence, citizenship, tax residency) make someone a U.S. person; and whether a holder with no country data on record must be refused (today: refused).

---

← [Test plan](test-plan.md) · [Index](README.md) · [7 · Model and diagrams](7-model/README.md) →
