#!/usr/bin/env bash
# Deploys the whole spike from scratch on Stellar testnet and writes deployments/testnet.json.
#
# Requires: stellar-cli >= 25.2, cargo, openssl, python3, and funded testnet identities
#   rwaxi-admin rwaxi-manager rwaxi-alice rwaxi-bob rwaxi-carol
# (create them with:  for n in admin manager alice bob carol; do stellar keys generate rwaxi-$n --network testnet --fund; done)
#
# Alice and Bob are Colombian residents, Carol is a U.S. resident. All three hold a signed KYC claim.
set -euo pipefail

cd "$(dirname "$0")/.."
NET=testnet
WASM=target/wasm32v1-none/release
KYC=1
PERIOD_LEDGERS=${PERIOD_LEDGERS:-$((17280 * 365))}   # placeholder: ~1 year; counsel sets the real length (ADR-007)
CLAIM_KEY=.secrets/claim-signer.key

addr() { stellar keys address "rwaxi-$1"; }
ADMIN=$(addr admin); MANAGER=$(addr manager)
ALICE=$(addr alice); BOB=$(addr bob); CAROL=$(addr carol)

deploy() { # <alias> <wasm name> <source> -- constructor args...
  local alias=$1 wasm=$2 source=$3; shift 3
  stellar contract deploy --network $NET --source "rwaxi-$source" --wasm "$WASM/$wasm.wasm" --alias "rwaxi-$alias-$RUN" "$@" 2>/dev/null | tail -n 1
}
call() { # <source> <contract id> -- fn args...
  local source=$1 id=$2; shift 2
  stellar contract invoke --network $NET --source "rwaxi-$source" --id "$id" "$@"
}

echo "== Build"
stellar contract build >/dev/null
[ -f "$CLAIM_KEY" ] || { mkdir -p .secrets; umask 077; openssl rand -hex 32 > "$CLAIM_KEY"; }
PUBKEY=$(cargo run -q -p sign-claim -- pubkey "$CLAIM_KEY")
RUN=$(date +%m%d%H%M%S)

echo "== Claim topics and issuers, claim issuer"
CTI=$(deploy cti claim_topics_and_issuers admin -- --admin "$ADMIN" --manager "$MANAGER")
call manager "$CTI" -- add_claim_topic --claim_topic $KYC --operator "$MANAGER" >/dev/null
ISSUER=$(deploy issuer claim_issuer admin -- --owner "$ADMIN")
call manager "$CTI" -- add_trusted_issuer --trusted_issuer "$ISSUER" --claim_topics "[$KYC]" --operator "$MANAGER" >/dev/null
call admin "$ISSUER" -- allow_key --public_key "$PUBKEY" --registry "$CTI" --claim_topic $KYC >/dev/null

echo "== Investor identities with signed KYC claims"
NOW=$(( $(date +%s) - 120 ))
declare -A IDENTITY
for who in alice bob carol; do
  id=$(deploy "identity-$who" investor_identity admin -- --owner "$(addr $who)")
  out=$(cargo run -q -p sign-claim -- sign "$CLAIM_KEY" "$ISSUER" "$id" $KYC 365 $NOW $NET)
  data=$(sed -n 's/^data=//p' <<<"$out"); sig=$(sed -n 's/^signature=//p' <<<"$out")
  call "$who" "$id" -- add_claim --topic $KYC --scheme 101 --issuer "$ISSUER" \
    --signature "$sig" --data "$data" --uri "https://rwaxi.com/spike/kyc/$who" >/dev/null
  IDENTITY[$who]=$id
done

echo "== Identity registry and verifier"
REGISTRY=$(deploy registry identity_registry admin -- --admin "$ADMIN" --manager "$MANAGER")
# CountryData { country: Individual(Residence(code)), metadata: None } as explicit ScVal JSON,
# because the registry takes `Vec<Val>` and the CLI cannot infer the struct type.
profile() { printf '[{"map":[{"key":{"symbol":"country"},"val":{"vec":[{"symbol":"Individual"},{"vec":[{"symbol":"Residence"},{"u32":%s}]}]}},{"key":{"symbol":"metadata"},"val":"void"}]}]' "$1"; }
call manager "$REGISTRY" -- add_identity --account "$ALICE" --identity "${IDENTITY[alice]}" --initial_profiles "$(profile 170)" --operator "$MANAGER" >/dev/null
call manager "$REGISTRY" -- add_identity --account "$BOB"   --identity "${IDENTITY[bob]}"   --initial_profiles "$(profile 170)" --operator "$MANAGER" >/dev/null
call manager "$REGISTRY" -- add_identity --account "$CAROL" --identity "${IDENTITY[carol]}" --initial_profiles "$(profile 840)" --operator "$MANAGER" >/dev/null
VERIFIER=$(deploy verifier identity_verifier admin -- --admin "$ADMIN" --manager "$MANAGER" \
  --identity_registry_storage "$REGISTRY" --claim_topics_and_issuers "$CTI")

echo "== Compliance and the Reg S module"
COMPLIANCE=$(deploy compliance compliance admin -- --admin "$ADMIN" --manager "$MANAGER")
MODULE=$(deploy regs reg_s_distribution_period admin -- --owner "$ADMIN" --compliance "$COMPLIANCE")
call manager "$COMPLIANCE" -- add_module_to --hook CanTransfer --module "$MODULE" --operator "$MANAGER" >/dev/null
call manager "$COMPLIANCE" -- add_module_to --hook CanCreate   --module "$MODULE" --operator "$MANAGER" >/dev/null

echo "== Token"
TOKEN=$(deploy token rwa_token admin -- --name "Avista Beach Fund I (spike)" --symbol ABF1 \
  --admin "$ADMIN" --manager "$MANAGER" --compliance "$COMPLIANCE" --identity_verifier "$VERIFIER")
call manager "$REGISTRY"   -- bind_token --token "$TOKEN" --operator "$MANAGER" >/dev/null
call manager "$COMPLIANCE" -- bind_token --token "$TOKEN" --operator "$MANAGER" >/dev/null
call admin "$MODULE" -- set_identity_registry_storage --token "$TOKEN" --irs "$REGISTRY" >/dev/null
LEDGER=$(stellar ledger latest --network $NET | sed -n 's/^Sequence: //p')
PERIOD_END=$((LEDGER + PERIOD_LEDGERS))
call admin "$MODULE" -- set_period_end --token "$TOKEN" --period_end_ledger $PERIOD_END >/dev/null

echo "== Mint 100 ABF1 to Alice"
call manager "$TOKEN" -- mint --to "$ALICE" --amount 1000000000 --operator "$MANAGER" >/dev/null

python3 -I - "$@" <<PY
import json, hashlib, subprocess, datetime
wasm = {n: hashlib.sha256(open(f"$WASM/{n}.wasm","rb").read()).hexdigest() for n in
  ["rwa_token","compliance","identity_registry","identity_verifier","claim_topics_and_issuers","claim_issuer","investor_identity","reg_s_distribution_period"]}
def git(*a):
    try: return subprocess.check_output(["git",*a],text=True).strip()
    except Exception: return None
doc = {
  "network": "testnet",
  "deployed_at": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
  "commit": git("rev-parse","HEAD"), "working_tree_clean": git("status","--porcelain")=="",
  "reg_s": {"period_end_ledger": $PERIOD_END, "deployed_at_ledger": $LEDGER, "note": "placeholder length; counsel defines the Rule 903 period"},
  "contracts": {"rwa_token":"$TOKEN","compliance":"$COMPLIANCE","reg_s_distribution_period":"$MODULE",
    "identity_registry":"$REGISTRY","identity_verifier":"$VERIFIER","claim_topics_and_issuers":"$CTI","claim_issuer":"$ISSUER",
    "identity_alice":"${IDENTITY[alice]}","identity_bob":"${IDENTITY[bob]}","identity_carol":"${IDENTITY[carol]}"},
  "accounts": {"admin":"$ADMIN","manager":"$MANAGER","alice_colombia":"$ALICE","bob_colombia":"$BOB","carol_usa":"$CAROL"},
  "wasm_sha256": wasm,
}
json.dump(doc, open("deployments/testnet.json","w"), indent=2); open("deployments/testnet.json","a").write("\n")
PY
echo "== Done. Contract IDs in deployments/testnet.json"
