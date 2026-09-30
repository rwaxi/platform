# Contributing

## Workflow

1. Open or pick an issue.
2. Branch from `main`: `feat/<short-name>`, `fix/<short-name>`, `docs/<short-name>`.
3. Small commits with clear messages, e.g. `feat(reg-s): block US recipients during period`.
4. Open a pull request using the template. CI must pass.
5. At least one approving review from another team member. Code owners review their areas.
6. Squash or rebase merge into `main`. No force-push to `main`.

## Requirements for contract changes

- Unit tests for every new rule, including boundary cases.
- `cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test` pass locally.
- Any testnet deploy records contract IDs and transaction hashes in `deployments/testnet.json`.
- Non-obvious design decisions get an ADR in `docs/6-decisions/`.

## AI-assisted contributions

State in the PR description whether AI tools were used and for what, and add a line to [AI_DISCLOSURE.md](AI_DISCLOSURE.md).
