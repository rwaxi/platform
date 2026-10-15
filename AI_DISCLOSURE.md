# AI Disclosure

This file lists every use of AI-generated or AI-assisted artifacts in this repository, how each was reviewed, and by whom. It is updated with every pull request where AI tools were used.

## Policy

- AI tools may assist with drafting documentation, diagrams, tests and code.
- Every AI-assisted artifact is reviewed by a named team member before merge, who is accountable for its content.
- Smart-contract code is never merged without human review, passing tests and CI.
- Each PR description states whether AI was used and for what.

## Log

Paths are the current ones. On 2026-09-30 the docs were reorganized into numbered folders.

| Date | Artifact | Tool | What the tool did | Reviewed by | Review performed |
|---|---|---|---|---|---|
| 2026-09-24 | `docs/2-overview.md`, `docs/3-c4/*`, `docs/4-flows.md`, `docs/5-security.md`, diagrams | Claude (Anthropic) | First draft of text and Mermaid diagrams from the team's technical brief and the OpenZeppelin `stellar-contracts` v0.7.1 source | _technical lead_ | _to complete_ |
| 2026-09-29 | `docs/7-model/rwaxi-architecture.archimate` | Claude (Anthropic) | Generated the Archi (ArchiMate) model — C4 context, containers and on-chain components views — from the architecture docs; layout is a first pass | _technical lead_ | _to complete: open in Archi, review elements/relations, adjust layout_ |
| 2026-09-29 | `docs/7-model/RWAXI_Architecture_Diagrams.pdf` | Claude (Anthropic) | Drafted the explanatory document; rebuilt on 2026-09-30 with the twelve Archi exports and shorter text from the technical lead's notes | _technical lead_ | _to complete: review text_ |
| 2026-09-29 | `docs/7-model/RWAXI_Arquitectura_Diagramas.pdf` | Claude (Anthropic) | Spanish version of the explanatory document, rebuilt on 2026-09-30 with the twelve Archi exports | _technical lead_ | _to complete_ |
| 2026-09-30 | `docs/7-model/rwaxi-architecture.archimate` (layout), `docs/7-model/diagrams/c4-*.png`, text of `docs/3-c4/*` §1 and `docs/2-overview.md` §4 | Claude (Anthropic) | Re-laid out the four views (C4-3 split in two), exported the PNGs with Archi 5.10 and drafted the short explanations from the technical lead's notes | _technical lead_ | _to complete_ |
| 2026-09-30 | Flow and DFD views in the Archi model, `docs/7-model/diagrams/flow-*.png` and `dfd-*.png`, text of `docs/4-flows.md` | Claude (Anthropic) | Moved the five flows and three DFDs from Mermaid to Archi views, exported them and drafted the short explanations | _technical lead_ | _to complete_ |
| 2026-09-24 | `docs/6-decisions/ADR-001` … `ADR-007` | Claude (Anthropic) | First draft of decision records and option analysis | _technical lead_ | _to complete_ |
| 2026-09-24 | `README.md`, `SECURITY.md`, `CONTRIBUTING.md`, templates | Claude (Anthropic) | First draft | _technical lead_ | _to complete_ |
| 2026-10-15 | `contracts/reg-s-distribution-period/` (`lib.rs`, `storage.rs`, `test.rs`) | Claude Code (Anthropic) | Wrote the Reg S compliance module and its 13 unit tests from ADR-007 and OpenZeppelin's `ComplianceModule` trait | _technical lead_ | _to complete: read the module line by line, run the tests_ |
| 2026-10-15 | `contracts/rwa-token`, `compliance`, `identity-registry`, `identity-verifier`, `claim-topics-and-issuers`, `claim-issuer`, `investor-identity`; `NOTICE` | Claude Code (Anthropic) | Copied OpenZeppelin's v0.7.2 RWA examples (MIT) into the workspace with an origin header, and added the country data getters to `identity-registry` | _technical lead_ | _to complete_ |
| 2026-10-15 | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | Claude Code (Anthropic) | Set up the workspace and pinned OpenZeppelin 0.7.2, soroban-sdk 26.1.1 and Rust 1.98.1 | _technical lead_ | _to complete_ |
| 2026-10-15 | `tests/integration/` | Claude Code (Anthropic) | Wrote the five end-to-end tests of the spike scenario | _technical lead_ | _to complete_ |
| 2026-10-15 | `tools/sign-claim/`, `scripts/deploy-testnet.sh`, `scripts/demo-blocked-transfer.sh`, `scripts/fetch-release-wasm.sh` | Claude Code (Anthropic) | Wrote the claim signing tool and the deploy, demo and release download scripts | _technical lead_ | _to complete_ |
| 2026-10-15 | `.github/workflows/ci.yml`, `.github/workflows/release.yml` | Claude Code (Anthropic) | Wrote the CI (fmt, clippy, tests, coverage, WASM build) and the SEP-55 release workflow | _technical lead_ | _to complete_ |
| 2026-10-15 | `deployments/testnet.json`, testnet contracts | Claude Code (Anthropic) | Ran the deployment and the demo on testnet from the `v0.1.1` release artifacts, with testnet-only keys created for the spike | _technical lead_ | _to complete: check the contracts and the transaction on Stellar Expert_ |
| 2026-10-15 | `README.md` (verification table, build steps, spike and award), `docs/test-plan.md`, `docs/compliance-matrix.md` | Claude Code (Anthropic) | Drafted the text from the code, the test results and the team's notes | _technical lead_ | _to complete_ |
| 2026-10-15 | `docs/2-overview.md`, `docs/3-c4/3-components.md`, ADR-001, ADR-007 | Claude Code (Anthropic) | Corrected versions and the compliance hook model after checking OpenZeppelin's source | _technical lead_ | _to complete_ |
