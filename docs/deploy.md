# Deployment guide

> Status: **v0.1** · 2026-10-15 · testnet. Mainnet differences are in §6.

How we put the spike on Stellar testnet, and how to check that what is deployed is what is in this repository. The script that does all of it is [`scripts/deploy-testnet.sh`](../scripts/deploy-testnet.sh).

## 1. What you need

- Rust 1.98.1 with the `wasm32v1-none` target (pinned in `rust-toolchain.toml`).
- `stellar-cli` 28.1.0. Plain `cargo build` cannot build these contracts: OpenZeppelin's crates use spec shaking v2, which only `stellar contract build` supports.
- `gh` logged in, to download release artifacts and check their attestations.
- Five testnet identities funded by friendbot:

```bash
for n in admin manager alice bob carol; do
  stellar keys generate rwaxi-$n --network testnet --fund
done
```

The script creates the claim-signing key in `.secrets/claim-signer.key` on first run. It is a testnet key and never leaves the machine (`.secrets/` is ignored by git).

## 2. Deploy from a release (the way we publish evidence)

```bash
RELEASE=v0.1.1 ./scripts/deploy-testnet.sh
```

1. Downloads the WASM of each contract from the GitHub release and checks its build attestation (`gh attestation verify --signer-repo stellar-expert/soroban-build-workflow`). If any check fails, nothing is deployed.
2. Deploys and wires the contracts in this order:

| # | Contract | Wired to |
|---|---|---|
| 1 | `claim-topics-and-issuers` | topic 1 (KYC) |
| 2 | `claim-issuer` | trusted for topic 1; signing key allowed |
| 3 | `investor-identity` × 3 | one per investor, with a signed KYC claim |
| 4 | `identity-registry` | wallet → identity, country of residence |
| 5 | `identity-verifier` | registry and topics |
| 6 | `compliance` | dispatcher |
| 7 | `reg-s-distribution-period` | registered on `CanTransfer` and `CanCreate` |
| 8 | `rwa-token` | compliance and verifier; bound to registry and dispatcher |

3. Sets the Reg S period end (placeholder: one year of ledgers), mints 100 ABF1 to Alice and writes `deployments/testnet.json` with contract IDs, WASM hashes, commit and source of the WASM.

Without `RELEASE`, the script builds locally. Use that only for development: the record then says `"wasm_source": "local build"` and explorers will not show a verified build.

## 3. Check the deployment

```bash
./scripts/demo-blocked-transfer.sh
```

Expected: the period is active, Alice → Bob succeeds with a transaction hash, Alice → Carol fails with `Error(Contract, #305)`. The same check runs in the [browser demo](https://rwaxi.github.io/platform/demo/).

On Stellar Expert, each contract page must show the build as verified from `github.com/rwaxi/platform` at the release commit.

## 4. Cut a release

1. Merge to `main` with CI green.
2. Tag: `git tag -a vX.Y.Z -m "…" && git push origin vX.Y.Z`.
3. `release.yml` builds each contract in GitHub Actions, attests it and publishes one release per contract (`vX.Y.Z-<package>_…`).
4. Deploy with `RELEASE=vX.Y.Z` and commit the new `deployments/testnet.json` and README links.

## 5. When testnet is reset

Testnet is wiped from time to time and every contract disappears. Fund the identities again and run §2 with the latest release; then update the README links. `scripts/redeploy-after-reset.sh` (task C-08) automates the check and the redeploy.

## 6. Mainnet (T3), what changes

- `admin` and `agent` are Stellar multisig accounts; each signer uses a hardware key. No single person can deploy, upgrade or move units.
- The claim-signing key lives in cloud KMS; the KYC service is the only identity allowed to use it.
- Roles split as in [the overview](2-overview.md): `admin`, `agent`, `claim_signer`, `module_operator`.
- The period end and restricted relations come from counsel and are recorded in [compliance-matrix.md](compliance-matrix.md) before deploying.
- The deployment is rehearsed on testnet with the same release, and the mainnet record is published next to the testnet one.

---

← [Threat model](threat-model.md) · [Index](README.md) · [Monitoring](monitoring.md) →
