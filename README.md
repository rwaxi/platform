# RWAXI Platform

Real-world-asset security tokens on Stellar. The transfer rules run inside the token contract on every transfer, not in a database the operator can edit.

[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

> **Status (September 2026): architecture published, testnet spike in progress.**
> First milestone is `v0.1.0-spike`: an OpenZeppelin RWA token on Stellar testnet with our Reg S compliance module, one transfer blocked by the rule and one allowed, both with public transaction hashes.

This repository is meant to be read in order. Each step takes a couple of minutes.

## 1 · What RWAXI is and how to verify it

Every `transfer`, `mint` and `burn` checks the identity of both parties against on-chain claims and then runs the vehicle's compliance modules in the same transaction. If one check fails, everything reverts.

![Compliance-checked transfer](docs/7-model/diagrams/flow-1-compliance-transfer.png)

| Verify on Stellar testnet | Link |
|---|---|
| RWA token contract | _available with v0.1.0-spike_ |
| Compliance dispatcher and Reg S module | _available with v0.1.0-spike_ |
| ❌ Transfer blocked (US person during the Reg S period) | _available with v0.1.0-spike_ |
| ✅ Transfer allowed | _available with v0.1.0-spike_ |
| Reproduce both with one command | `./scripts/demo-blocked-transfer.sh` (with v0.1.0-spike) |

## 2 · The central decision → [Architecture overview](docs/2-overview.md)

Why the rules live in the token, what the spike covers, what the grant pays for (tranches T1 to T3) and what stays outside it.

## 3 · The system, from the outside in → [C4 model](docs/3-c4/1-context.md)

| | View | What it answers |
|---|---|---|
| 3.1 | [Context](docs/3-c4/1-context.md) | Who uses RWAXI and which outside systems we depend on |
| 3.2 | [Containers](docs/3-c4/2-containers.md) | What we deploy, where it runs and how the pieces talk |
| 3.3 | [Components](docs/3-c4/3-components.md) | The contracts of each vehicle, and how compliance and identity work |

![Containers](docs/7-model/diagrams/c4-2-containers.png)

## 4 · How it works in motion → [Critical flows](docs/4-flows.md)

Compliance-checked transfer (the spike), onboarding with passkey, subscription, distribution and lost-device recovery.

## 5 · Security → [Data flows and trust boundaries](docs/5-security.md)

Where each piece of data travels, where it is stored and which of the six trust boundaries it crosses. Input to the STRIDE threat model.

## 6 · Why we decided this way → [ADRs](docs/6-decisions/README.md)

ADR-001 to ADR-007, each with context, options and decision. ADR-007 is the Reg S rule of the spike.

## 7 · Where the diagrams come from → [Model and diagrams](docs/7-model/README.md)

One Archi model generates the twelve diagrams. Also here: the architecture PDF in [English](docs/7-model/RWAXI_Architecture_Diagrams.pdf) and [Spanish](docs/7-model/RWAXI_Arquitectura_Diagramas.pdf).

## 8 · Open source and program requirements

| File | |
|---|---|
| [LICENSE](LICENSE) | Apache-2.0 for contracts and SDK |
| [SECURITY.md](SECURITY.md) | How to report a vulnerability. Don't open public issues for them |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How we work: branches, reviews, ADRs |
| [.github/](.github) | Code owners, PR and task templates |

## What comes next

- From 5 October: `contracts/`, `tests/`, `scripts/` and `deployments/`, with CI and a coverage badge.
- By 23 October: the two transaction hashes on testnet and the `v0.1.0-spike` release with a verified build (SEP-55).

Built on [OpenZeppelin `stellar-contracts`](https://github.com/OpenZeppelin/stellar-contracts) (RWA module, smart accounts), Soroban, native USDC and OpenZeppelin Relayer.
