# AI Disclosure

We use an AI coding assistant (Claude Code, by Anthropic) the way we use any other tool: it drafts, we decide. This file lists every artifact it helped produce, what it did, and who reviewed it. It is updated with every pull request where AI was used.

## How we work

- The assistant drafts documentation, diagrams, tests and code. A named team member reviews each artifact and answers for its content.
- No contract code reaches `main` without the technical lead's review, passing tests and CI.
- Each pull request says whether AI was used and for what.
- A row without a reviewer means the review is still pending, not that it was skipped.

## Log

Paths are the current ones. On 2026-09-30 the docs were reorganized into numbered folders.

| Date | Artifact | Tool | What the tool did | Reviewed by | Review performed |
|---|---|---|---|---|---|
| 2026-09-24 | `docs/2-overview.md`, `docs/3-c4/*`, `docs/4-flows.md`, `docs/5-security.md`, diagrams | Claude (Anthropic) | First draft of text and Mermaid diagrams from the team's technical brief and the OpenZeppelin `stellar-contracts` v0.7.1 source | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-09-29 | `docs/7-model/rwaxi-architecture.archimate` | Claude (Anthropic) | Generated the Archi (ArchiMate) model — C4 context, containers and on-chain components views — from the architecture docs; layout is a first pass | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-09-29 | `docs/7-model/RWAXI_Architecture_Diagrams.pdf` | Claude (Anthropic) | Drafted the explanatory document; rebuilt on 2026-09-30 with the twelve Archi exports and shorter text from the technical lead's notes | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-09-29 | `docs/7-model/RWAXI_Arquitectura_Diagramas.pdf` | Claude (Anthropic) | Spanish version of the explanatory document, rebuilt on 2026-09-30 with the twelve Archi exports | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-09-30 | `docs/7-model/rwaxi-architecture.archimate` (layout), `docs/7-model/diagrams/c4-*.png`, text of `docs/3-c4/*` §1 and `docs/2-overview.md` §4 | Claude (Anthropic) | Re-laid out the four views (C4-3 split in two), exported the PNGs with Archi 5.10 and drafted the short explanations from the technical lead's notes | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-09-30 | Flow and DFD views in the Archi model, `docs/7-model/diagrams/flow-*.png` and `dfd-*.png`, text of `docs/4-flows.md` | Claude (Anthropic) | Moved the five flows and three DFDs from Mermaid to Archi views, exported them and drafted the short explanations | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-09-24 | `docs/6-decisions/ADR-001` … `ADR-007` | Claude (Anthropic) | First draft of decision records and option analysis | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-09-24 | `README.md`, `SECURITY.md`, `CONTRIBUTING.md`, templates | Claude (Anthropic) | First draft | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `contracts/reg-s-distribution-period/` (`lib.rs`, `storage.rs`, `test.rs`) | Claude Code (Anthropic) | Wrote the Reg S compliance module and its 13 unit tests from ADR-007 and OpenZeppelin's `ComplianceModule` trait | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `contracts/rwa-token`, `compliance`, `identity-registry`, `identity-verifier`, `claim-topics-and-issuers`, `claim-issuer`, `investor-identity`; `NOTICE` | Claude Code (Anthropic) | Copied OpenZeppelin's v0.7.2 RWA examples (MIT) into the workspace with an origin header, and added the country data getters to `identity-registry` | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | Claude Code (Anthropic) | Set up the workspace and pinned OpenZeppelin 0.7.2, soroban-sdk 26.1.1 and Rust 1.98.1 | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `tests/integration/` | Claude Code (Anthropic) | Wrote the five end-to-end tests of the spike scenario | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `tools/sign-claim/`, `scripts/deploy-testnet.sh`, `scripts/demo-blocked-transfer.sh`, `scripts/fetch-release-wasm.sh` | Claude Code (Anthropic) | Wrote the claim signing tool and the deploy, demo and release download scripts | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `.github/workflows/ci.yml`, `.github/workflows/release.yml` | Claude Code (Anthropic) | Wrote the CI (fmt, clippy, tests, coverage, WASM build) and the SEP-55 release workflow | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `deployments/testnet.json`, testnet contracts | Claude Code (Anthropic) | Ran the deployment and the demo on testnet from the `v0.1.1` release artifacts, with testnet-only keys created for the spike | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `README.md` (verification table, build steps, spike and award), `docs/test-plan.md`, `docs/compliance-matrix.md` | Claude Code (Anthropic) | Drafted the text from the code, the test results and the team's notes | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `docs/2-overview.md`, `docs/3-c4/3-components.md`, ADR-001, ADR-007 | Claude Code (Anthropic) | Corrected versions and the compliance hook model after checking OpenZeppelin's source | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `demo/` (`index.html`, `demo.js`), `.github/workflows/pages.yml` | Claude Code (Anthropic) | Wrote the browser demo that simulates both transfers through the public RPC, tested it against testnet in Chrome, and the Pages workflow | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `docs/threat-model.md`, `docs/deploy.md`, `docs/monitoring.md` | Claude Code (Anthropic) | Drafted the STRIDE model, the deployment guide and the monitoring plan from the code, the deployment and OpenZeppelin's event names | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-15 | `docs/metrics.md`, `docs/roadmap.md`, `docs/5-security.md` (flow D14), `docs/2-overview.md` (OZ modules row) | Claude Code (Anthropic) | Drafted metrics and the verification table per tranche; corrected two statements | Carlos Serna, technical lead | Reviewed and approved, 2026-10-15 |
| 2026-10-19 | ADR-007 (parameters, acceptance criteria), `docs/compliance-matrix.md` v0.2, `docs/monitoring.md` (period check) | Claude Code (Anthropic) | Drafted the Reg S parameters and the baseline for R-03 to R-14 from the text of Rules 902 to 905, Rule 144 and the OFAC regulations, while counsel's rules are pending. Not legal advice | | |
| 2026-10-20 | `docs/errors.md` | Claude Code (Anthropic) | Helped Stiven (stevenrome) collect the error codes and check each number and call site in the OpenZeppelin 0.7.2 source and the Reg S module | | |
