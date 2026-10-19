# Compliance matrix

> Status: **v0.2** · 2026-10-19 · R-01 and R-02 are built. For the other rows we wrote a baseline from the text of the regulation while we wait for securities counsel (rules were due 2026-10-09). A baseline is our reading, not legal advice. A row moves to "done" only after counsel confirms it.

Each row takes one legal requirement to the place where the chain enforces it: the contract or module, the parameter that encodes it, and the test that proves it. A rule is "done" only when all four columns are filled and the test runs in CI.

## 1. How a rule is enforced

A rule can live in one of three places. Which one is a design choice per rule, recorded in the last column of §2.

| Place | Enforced by | Good for |
|---|---|---|
| **Identity** | Signed claims in the investor's identity contract, checked by the identity verifier on every transfer and mint (`verify_identity`) | Who may hold the token at all: KYC passed, investor type, claim expiry |
| **Compliance module** | A contract registered on the compliance dispatcher. `CanTransfer` / `CanCreate` hooks can reject; `Transferred` / `Created` / `Destroyed` hooks keep the module's own state | Rules about the transfer itself: country of the recipient, dates, balances, supply, volumes |
| **Off-chain** | The backend or the offering documents, before anything reaches the chain | Rules the chain cannot see: suitability questionnaires, source of funds documents, minimum ticket in the subscription flow |

Forced and recovery transfers skip the compliance modules (the token does not call `can_transfer` on them) but recovery still verifies the identity of the new wallet.

## 2. Matrix · Avista Beach Tokenization Fund I

Working assumptions, the same as in ADR-007: the issuer is a U.S. entity that does not report under the Exchange Act, the units are equity, and Fund I is sold only under Regulation S, mainly to investors in Colombia.

| # | Requirement | Source | Where | Contract / module | Parameter | Test | Status |
|---|---|---|---|---|---|---|---|
| R-01 | During the distribution compliance period, no offer or sale to a U.S. person | Reg S, Rules 902(f), 902(k), 903(b)(3) | Module | `reg-s-distribution-period` (ours, [ADR-007](6-decisions/ADR-007-spike-compliance-rule-reg-s.md)) | `period_end_ledger` = one year after the distribution is complete; `restricted_relations` = `[Residence]` | `test-plan.md` §3, criteria 1–6 | **Built, on testnet.** Parameters from the regulation, counsel to confirm |
| R-02 | Every holder has passed KYC with an approved provider | Issuer policy; needed for R-03 and R-05 | Identity | claim topic `1` (KYC), trusted `claim-issuer` | topic id, issuer, signing key, claim validity | `a_wallet_without_identity_cannot_receive` | **Built** with one test issuer and key. Provider (task 41) and validity period pending |
| R-03 | No holder on a sanctions list, and no holder in a comprehensively sanctioned jurisdiction | OFAC regulations (31 CFR Ch. V) bind the issuer as a U.S. entity | Identity, plus module for countries | KYC provider screens before the claim is signed; `country_restrict` (OZ) with the sanctioned jurisdictions | list of country codes, kept off chain and loaded on the module | — | Baseline. The country list changes with OFAC, so it is data, not code. Counsel confirms whether Colombian AML rules add anything |
| R-04 | Countries where Fund I may be offered | Each country's securities law. Reg S does not limit other countries | Module | `country_allow` (OZ) | Colombia (170) to start | — | Baseline. Counsel says how the offer is made in Colombia (private offer or a licensed local promoter) and which other countries are open |
| R-05 | The buyer certifies it is not a U.S. person and agrees to resell only under Reg S | Rule 903(b)(3)(iii)(B)(1) and (2) | Identity | claim topic "Reg S certification", signed when the subscription documents are signed | topic id | — | Baseline. Reg S does not ask for accreditation; counsel says if Colombia asks for an investor type |
| R-06 | Minimum and maximum investment per investor | Offering memorandum (business terms, not Reg S) | Off-chain (minimum) · Module (maximum) | subscription flow (T2) · `max_balance` (OZ) | amounts in token units | — | Waiting for the memorandum |
| R-07 | Holding period after subscription | Covered by R-01 and R-09 under Reg S | Module | `initial_lockup_period` (OZ) only if the memorandum adds a lock-up of its own | lock-up length in ledgers | — | Not needed by the regulation; only if the memorandum asks |
| R-08 | Maximum size of the offering | Offering memorandum | Module | `supply_limit` (OZ) | cap in token units | — | Waiting for the memorandum |
| R-09 | After the period, a resale to a U.S. person needs an exemption | Rule 905 (restricted securities); Rule 144(b)(1) and (d)(1)(ii) | Identity + module | claim topic "affiliate"; custom module or `transfer_allow` (OZ) for affiliate transfers | affiliate list | — | Baseline. Holders that are not affiliates: free after one year, the same as R-01. Affiliates (managers, 10 % holders, the sponsor) keep Rule 144 limits. Counsel confirms who is an affiliate |
| R-10 | Limits on transfer volume per period | Rule 144(e) volume limits, affiliates only | Module | `time_transfers_limits` (OZ) | window and limit | — | Only if R-09 leaves affiliates able to sell into the U.S. |
| R-11 | Maximum number of holders | Investment Company Act 3(c)(1) if the fund relies on it (100 owners); Colombian private offer if it goes that way | Module | none in OZ; custom module (T1) | holder cap | — | Counsel says which exemption Fund I uses. A fund that owns real estate directly may not be an investment company at all |
| R-12 | Recovery of a lost wallet | Issuer procedure | Identity + token | `recover_identity` (registry) then `recover_balance` (token), operator role | — | _planned_ T1 (`test-plan.md` §6) | Procedure needs JP's legal approval |
| R-13 | Legend on the units: transfers only under Reg S | Rule 903(b)(3)(iii)(B)(3) | Off-chain + metadata | subscription documents; token name or metadata URI points to the legend | URI | — | Baseline. Counsel approves the text |
| R-14 | No directed selling efforts in the U.S. | Rule 903(a)(2), Rule 902(c) | Off-chain | website and campaigns: no U.S. targeting, a Reg S notice, U.S. visitors cannot start a subscription | — | — | Baseline. Applies to marketing, not to contracts |

The mechanism in each row is a candidate. Counsel's answer can move a rule to another place or remove it.

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

We no longer ask for the rules from scratch. We ask counsel to confirm or correct our reading:

1. Fund I is Category 3: U.S. non-reporting issuer, equity. One-year period that starts when the distribution is complete.
2. A U.S. person is decided by residence only (Rule 902(k)). A holder with no country data is refused during the period.
3. Measuring the period in ledgers, with the weekly check in `monitoring.md`, is acceptable.
4. Who is an affiliate (R-09), and whether the fund relies on an exemption with a holder cap (R-11).
5. How the offer reaches Colombian investors and whether Colombian rules add investor type, AML or holder limits (R-03, R-04, R-05, R-11).
6. The legend text (R-13).

---

← [Test plan](test-plan.md) · [Index](README.md) · [On-chain metrics](metrics.md) →
