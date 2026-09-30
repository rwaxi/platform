# ADR-006 — One token and compliance set per vehicle, shared identity

- Status: Proposed · 2026-09-24

## Context
Each fund is a separate legal vehicle with its own offering documents and rules. Investors may invest in several vehicles.

## Options
1. One multi-asset contract for all vehicles.
2. One token + compliance dispatcher (+ subscription, distribution) per vehicle; identity stack shared.

## Decision
Option 2, deployed by `vehicle-factory`.

## Consequences
- Legal isolation mirrors contract isolation; a pause or freeze on one vehicle does not touch others.
- Investors complete KYC once; claims are reused across vehicles, each vehicle declares which claim topics it requires.
- Compliance module contracts are shared, their configuration is per token (OZ modules are multi-token by design).
