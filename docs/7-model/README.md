# 7 · Model and diagrams

All the diagrams in this documentation come from one Archi model, [`rwaxi-architecture.archimate`](rwaxi-architecture.archimate) (Archi 5). That keeps the names consistent across views.

| View | Image | Used in |
|---|---|---|
| C4-1 System context | [c4-1-context.png](diagrams/c4-1-context.png) | [3.1](../3-c4/1-context.md) |
| C4-2 Containers | [c4-2-containers.png](diagrams/c4-2-containers.png) | [3.2](../3-c4/2-containers.md) |
| C4-3a Vehicle contracts | [c4-3a-vehicle-contracts.png](diagrams/c4-3a-vehicle-contracts.png) | [3.3](../3-c4/3-components.md) |
| C4-3b Compliance and identity | [c4-3b-compliance-identity.png](diagrams/c4-3b-compliance-identity.png) | [3.3](../3-c4/3-components.md) |
| Flow 1 · Compliance-checked transfer | [flow-1-compliance-transfer.png](diagrams/flow-1-compliance-transfer.png) | [4](../4-flows.md) |
| Flow 2 · Onboarding | [flow-2-onboarding.png](diagrams/flow-2-onboarding.png) | [4](../4-flows.md) |
| Flow 3 · Subscription | [flow-3-subscription.png](diagrams/flow-3-subscription.png) | [4](../4-flows.md) |
| Flow 4 · Distribution | [flow-4-distribution.png](diagrams/flow-4-distribution.png) | [4](../4-flows.md) |
| Flow 5 · Recovery | [flow-5-recovery.png](diagrams/flow-5-recovery.png) | [4](../4-flows.md) |
| DFD 1a · Off-chain | [dfd-1a-offchain.png](diagrams/dfd-1a-offchain.png) | [5](../5-security.md) |
| DFD 1b · On-chain | [dfd-1b-onchain.png](diagrams/dfd-1b-onchain.png) | [5](../5-security.md) |
| DFD 2 · Spike | [dfd-2-spike.png](diagrams/dfd-2-spike.png) | [5](../5-security.md) |

Colors are the same everywhere: yellow for people, blue for what we build or configure, orange for contracts we write ourselves, grey for systems we don't control.

## Changing a diagram

Change the model, never the PNG.

1. Open `rwaxi-architecture.archimate` in Archi 5 and edit the view.
2. Export with *Report › HTML*, or from the command line:
   ```
   Archi -application com.archimatetool.commandline.app -nosplash \
     --loadModel docs/7-model/rwaxi-architecture.archimate --html.createReport /tmp/rwaxi-report
   ```
3. Copy the view images from `/tmp/rwaxi-report/<model-id>/images/` to `diagrams/` with the names in the table.

## PDF

The same twelve views with a short explanation each, for reading outside GitHub: [English](RWAXI_Architecture_Diagrams.pdf) · [Spanish](RWAXI_Arquitectura_Diagramas.pdf).

---

← [6 · Decisions](../6-decisions/README.md) · [Index](../README.md) · [8 · Open source files](../../README.md#8--open-source-and-program-requirements) →
