# Belladonna Play

Belladonna Play is a runtime security and integrity layer for games and interactive applications. Our aim is simple: help studios ship fair‑play experiences and protect creator value without punishing legitimate players or locking teams into heavy, opaque systems.

## Why Belladonna Play exists
- Fair play matters. Communities thrive when competition is authentic and creators can trust their builds in the wild.
- Respect for players and their machines. We favor least‑privilege designs, clear behavior, and minimum friction. No kernel drivers required; advanced monitoring (e.g., eBPF) is feature‑gated and opt‑in.
- Developer‑first. Teams deserve predictable performance, observable signals, and tooling that fits existing pipelines.

## Current status
- Private repository during an initial closed‑source phase.
- Internal version: v0.0.1 foundation (Linux‑only).
- Binaries‑only pre‑releases will be published as we stabilize the foundation.
- We plan to open‑source the HAL layer and selected developer tooling in a dedicated repository; runtime anti‑cheat and DRM components will remain closed‑source for now.

## What’s in the foundation today
- HAL‑unified Linux backend: capability and sandbox reporting (seccomp, namespaces), anti‑debug probes via libc routing.
- System monitoring: perf‑based baseline with eBPF loader scaffold (opt‑in) and mode reporting.
- DRM building blocks: encrypted entitlements cache, asset packing/decryption, mod signing/verification and allowlists.
- Observability: Prometheus‑style metrics and a simple capability JSON surface for diagnostics.

## Coming soon
- First‑party engine plugins for easier integration:
	- Unreal Engine
	- Unity
	- Godot
- Lightweight SDKs and example projects.
- Expanded dashboards and diagnostics for operations teams.
- Broader platform coverage as the Linux foundation hardens.

## Security and privacy commitments
- Least privilege by default; explicit, documented escalations when needed.
- Transparency over obscurity: clear surfaces, predictable behavior, and measurable signals.
- Data minimization: collect only what we must to uphold integrity and safety goals.
- Offline‑friendly operation where practical.

## Releases
- Target: Linux x86_64 (glibc ≥ 2.31; kernel ≥ 5.10 recommended).
- Verify artifacts using the accompanying SHA‑256 file before distribution.

## Getting updates and contact
- We’ll post updates here as the HAL/tooling open‑source plan progresses and as engine plugins land.
- For collaboration or evaluation inquiries, please contact the maintainers through this repository.
