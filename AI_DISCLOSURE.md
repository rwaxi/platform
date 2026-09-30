# AI Disclosure

This file lists every use of AI-generated or AI-assisted artifacts in this repository, how each was reviewed, and by whom. It is updated with every pull request where AI tools were used.

## Policy

- AI tools may assist with drafting documentation, diagrams, tests and code.
- Every AI-assisted artifact is reviewed by a named team member before merge, who is accountable for its content.
- Smart-contract code is never merged without human review, passing tests and CI.
- Each PR description states whether AI was used and for what.

## Log

| Date | Artifact | Tool | What the tool did | Reviewed by | Review performed |
|---|---|---|---|---|---|
| 2026-09-24 | `docs/architecture/*`, `docs/dataflow.md`, diagrams | Claude (Anthropic) | First draft of text and Mermaid diagrams from the team's technical brief and the OpenZeppelin `stellar-contracts` v0.7.1 source | _technical lead_ | _to complete_ |
| 2026-09-29 | `docs/architecture/archi/rwaxi-architecture.archimate` | Claude (Anthropic) | Generated the Archi (ArchiMate) model — C4 context, containers and on-chain components views — from `docs/architecture/*.md`; layout is a first pass | _technical lead_ | _to complete: open in Archi, review elements/relations, adjust layout_ |
| 2026-09-29 | `docs/architecture/RWAXI_Architecture_Diagrams.pdf` | Claude (Anthropic) | Drafted the explanatory document; rebuilt on 2026-09-30 with the twelve Archi exports and shorter text from the technical lead's notes | _technical lead_ | _to complete: review text_ |
| 2026-09-29 | `docs/architecture/RWAXI_Arquitectura_Diagramas.pdf` | Claude (Anthropic) | Spanish version of the explanatory document, rebuilt on 2026-09-30 with the twelve Archi exports | _technical lead_ | _to complete_ |
| 2026-09-30 | `docs/architecture/archi/rwaxi-architecture.archimate` (layout), `docs/architecture/diagrams/c4-*.png`, text of `c4-context.md`, `c4-containers.md`, `c4-components.md` §1 and `architecture/README.md` §4 | Claude (Anthropic) | Re-laid out the four views (C4-3 split in two), exported the PNGs with Archi 5.10 and drafted the short explanations from the technical lead's notes | _technical lead_ | _to complete_ |
| 2026-09-30 | Flow and DFD views in the Archi model, `docs/architecture/diagrams/flow-*.png` and `dfd-*.png`, text of `sequences.md` | Claude (Anthropic) | Moved the five flows and three DFDs from Mermaid to Archi views, exported them and drafted the short explanations | _technical lead_ | _to complete_ |
| 2026-09-24 | `docs/adr/ADR-001` … `ADR-007` | Claude (Anthropic) | First draft of decision records and option analysis | _technical lead_ | _to complete_ |
| 2026-09-24 | `README.md`, `SECURITY.md`, `CONTRIBUTING.md`, templates | Claude (Anthropic) | First draft | _technical lead_ | _to complete_ |
