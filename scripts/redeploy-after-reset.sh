#!/usr/bin/env bash
# Run after a Stellar testnet reset. Checks whether the token in deployments/testnet.json is
# still on testnet; only if it is gone, funds the rwaxi-* identities, deploys the latest
# release with deploy-testnet.sh and runs the demo.
#   ./scripts/redeploy-after-reset.sh
# Requires: stellar-cli, gh, and for a redeploy the identities
#   rwaxi-admin rwaxi-manager rwaxi-alice rwaxi-bob rwaxi-carol (see deploy-testnet.sh).
set -euo pipefail
cd "$(dirname "$0")/.."

REPO=${REPO:-rwaxi/platform}
j() { python3 -I -c "import json,sys; d=json.load(open('deployments/testnet.json')); print(eval(sys.argv[1]))" "$1"; }
TOKEN=$(j "d['contracts']['rwa_token']"); ADMIN=$(j "d['accounts']['admin']")

echo "== Is the token still on testnet? ($TOKEN)"
# Read-only simulation: nothing is signed or sent, so no secret key is needed.
if out=$(stellar contract invoke --network testnet --id "$TOKEN" --source "$ADMIN" --send=no -- decimals 2>&1); then
  echo "Yes (decimals: $out). Testnet was not reset; nothing to do."
  exit 0
fi
if ! grep -q "Contract not found" <<<"$out"; then
  echo "Could not check the token, so nothing was changed:" >&2
  echo "$out" >&2
  exit 1
fi
echo "No. Testnet was reset."

for n in admin manager alice bob carol; do
  stellar keys address "rwaxi-$n" >/dev/null 2>&1 || { echo "Missing identity rwaxi-$n; nothing was changed." >&2; exit 1; }
done

echo "== Fund the identities"
for n in admin manager alice bob carol; do
  stellar keys fund "rwaxi-$n" --network testnet
done

# Release tags look like v0.1.1-rwa-token_rwa-token_pkg0.1.0_cli28.1.0; deploy-testnet.sh wants v0.1.1.
TAG=$(gh release list -R "$REPO" --limit 1 --json tagName -q '.[0].tagName')
TAG=${TAG%%-*}
echo "== Deploy release $TAG"
RELEASE=$TAG ./scripts/deploy-testnet.sh

echo "== Demo"
./scripts/demo-blocked-transfer.sh

echo "== New contract IDs"
python3 -I -c 'import json; [print(f"{k}: {v}") for k, v in json.load(open("deployments/testnet.json"))["contracts"].items()]'

echo
echo "Update the contract links in README.md and contracts/*/README.md by hand, and commit deployments/testnet.json."
