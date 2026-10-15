#!/usr/bin/env bash
# Downloads each contract's WASM from a GitHub release made by release.yml and checks
# its build attestation (SEP-55). Writes <crate_name>.wasm files into <out-dir>.
#   ./scripts/fetch-release-wasm.sh v0.1.1 /tmp/wasm
set -euo pipefail
TAG=$1; OUT=$2; REPO=${REPO:-rwaxi/platform}
PACKAGES="rwa-token compliance identity-registry identity-verifier claim-topics-and-issuers claim-issuer investor-identity reg-s-distribution-period"
mkdir -p "$OUT"
for pkg in $PACKAGES; do
  release=$(gh release list -R "$REPO" --limit 200 --json tagName -q ".[].tagName | select(startswith(\"$TAG-$pkg___\"))" | head -n 1)
  [ -n "$release" ] || { echo "no release for $pkg at $TAG" >&2; exit 1; }
  tmp=$(mktemp -d)
  gh release download "$release" -R "$REPO" -p '*.wasm' -D "$tmp"
  file=$(ls "$tmp"/*.wasm)
  gh attestation verify "$file" -R "$REPO" >/dev/null
  mv "$file" "$OUT/${pkg//-/_}.wasm"
  echo "$pkg: $(sha256sum "$OUT/${pkg//-/_}.wasm" | cut -c1-16)… attested ($release)"
done
