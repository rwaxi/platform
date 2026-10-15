# RWAXI Platform documentation

RWAXI issues and administers real-world-asset security tokens on Stellar. The transfer rules run inside the token contract on every operation. First vehicle: Avista Beach Tokenization Fund I.

Read it in order. The numbers match the folders and files.

| Step | Read | In short |
|---|---|---|
| 2 | [Architecture overview](2-overview.md) | The central decision, scope of spike and grant, components by tranche |
| 3 | [C4 model](3-c4/1-context.md): [context](3-c4/1-context.md), [containers](3-c4/2-containers.md), [components](3-c4/3-components.md) | The system from the outside in |
| 4 | [Critical flows](4-flows.md) | The five flows, starting with the one the spike proves |
| 5 | [Security](5-security.md) | Data flows, data stores and trust boundaries |
| 6 | [Decisions](6-decisions/README.md) | ADR-001 to ADR-007 |
| 7 | [Model and diagrams](7-model/README.md) | The Archi model, the twelve images and the PDFs |
| — | [Test plan](test-plan.md) | What the spike tests, how to re-run it, and what is missing |
| — | [Compliance matrix](compliance-matrix.md) | Each legal rule mapped to the contract, parameter and test that enforce it |

Step 1 is the repository [README](../README.md) and step 8 the open source files at the root (license, security policy, contributing guide).
