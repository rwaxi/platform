#!/usr/bin/env bash
# Reproduces the spike evidence on testnet with one command:
#   Alice (Colombia) -> Bob (Colombia): allowed, produces a transaction hash.
#   Alice (Colombia) -> Carol (USA):    rejected by the Reg S compliance module while the period is active.
# Reads deployments/testnet.json. Needs the rwaxi-alice identity (see deploy-testnet.sh).
set -euo pipefail
cd "$(dirname "$0")/.."

j() { python3 -I -c "import json,sys; d=json.load(open('deployments/testnet.json')); print(eval(sys.argv[1]))" "$1"; }
TOKEN=$(j "d['contracts']['rwa_token']"); MODULE=$(j "d['contracts']['reg_s_distribution_period']")
ALICE=$(j "d['accounts']['alice_colombia']"); BOB=$(j "d['accounts']['bob_colombia']"); CAROL=$(j "d['accounts']['carol_usa']")
AMOUNT=${AMOUNT:-100000000}   # 10 ABF1 (7 decimals)
inv() { stellar contract invoke --network testnet --id "$1" "${@:2}"; }
bal() { inv "$TOKEN" --source rwaxi-alice -- balance --account "$1" 2>/dev/null | tr -d '"'; }

echo "Reg S period active: $(inv "$MODULE" --source rwaxi-alice -- is_active --token "$TOKEN" 2>/dev/null) (ends at ledger $(inv "$MODULE" --source rwaxi-alice -- period_end --token "$TOKEN" 2>/dev/null))"
echo "Balances before: alice=$(bal "$ALICE") bob=$(bal "$BOB") carol=$(bal "$CAROL")"

echo; echo "1) Alice -> Bob (both resident in Colombia): expected to succeed"
inv "$TOKEN" --source rwaxi-alice -- transfer --from "$ALICE" --to "$BOB" --amount "$AMOUNT" 2>&1 | grep -E "stellar.expert|Success|error" || true

echo; echo "2) Alice -> Carol (resident in the USA): expected to be rejected"
if out=$(inv "$TOKEN" --source rwaxi-alice -- transfer --from "$ALICE" --to "$CAROL" --amount "$AMOUNT" 2>&1); then
  echo "UNEXPECTED: the transfer to the U.S. investor went through"; echo "$out"; exit 1
fi
echo "$out" | grep -E "error:|Error\(Contract|can_transfer|TransferNotCompliant" | head -5

echo; echo "Balances after:  alice=$(bal "$ALICE") bob=$(bal "$BOB") carol=$(bal "$CAROL")"
