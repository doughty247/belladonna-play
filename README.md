# Belladonna Play

Belladonna Play is a runtime security and integrity layer—both DRM and anti-cheat—for games and interactive applications. My aim is simple: help studios ship fair-play experiences and protect creator value without punishing legitimate players or locking teams into heavy, opaque systems. I also support verified community mods through signing and allowlists so creators can embrace healthy mod ecosystems with confidence.

---

## Why Belladonna Play Exists

- **Fair play matters.** Communities thrive when competition is authentic and creators can trust their builds in the wild.  
- **Respect for players and their machines.** I favor least-privilege designs, clear behavior, and minimum friction. No kernel drivers required; advanced monitoring (e.g., eBPF) is feature-gated and opt-in.  
- **Developer-first.** Teams deserve predictable performance, observable signals, and tooling that fits existing pipelines.  

---

## Why the Name

- **“Belladonna”** is a potent flowering plant, and in Italian it also means “beautiful woman.” I embrace that duality—beauty and care on the surface with disciplined strength underneath.  
- Like the plant, **if misused it is deadly**: safe and invisible for fair players, but toxic for attackers and cheaters.  
- **“Play”** highlights the focus on games, fairness, and verified community mods.  
- Together, the name stands for **trust, strength, and protection applied responsibly.**

---

## Current Status

- Private repository during an initial closed-source phase.  
- Internal version: v0.0.1 foundation (Linux-only).  
- Binaries-only pre-releases will be published as I stabilize the foundation.  
- I plan to open-source the HAL layer and selected developer tooling in a dedicated repository; runtime anti-cheat and DRM components will remain closed-source for now.  

---

## What’s in the Foundation Today

- **HAL-unified Linux backend:** capability and sandbox reporting (seccomp, namespaces), anti-debug probes via libc routing.  
- **System monitoring:** perf-based baseline with eBPF loader scaffold (opt-in) and mode reporting.  
- **DRM building blocks:** encrypted entitlements cache, asset packing/decryption, mod signing/verification and allowlists.  
- **Observability:** Prometheus-style metrics and a simple capability JSON surface for diagnostics.  

---

## What is the HAL

The HAL (Host Abstraction Layer) is a thin, cross-platform interface that standardizes how Belladonna Play interacts with the operating system. In practice, it provides:

- Consistent contracts for process hardening, sandbox/capability reporting, and runtime monitoring across OSes.  
- A user-mode, least-privilege design with predictable behavior and clear observability.  
- A portability layer so SDKs and engine plugins integrate once and run consistently on supported platforms.  

The HAL focuses on stable, minimal surfaces. Advanced probes (e.g., eBPF on Linux) are feature-gated and opt-in. Implementation specifics are kept internal while I evaluate open-sourcing the HAL and related tooling.  

---

## Coming Soon

- First-party engine plugins for easier integration:  
  - Unreal Engine  
  - Unity  
  - Godot  
- Windows support  
- Lightweight SDKs and example projects  
- Expanded dashboards and diagnostics for operations teams  
- Broader platform coverage as the Linux foundation hardens  

---

## Windows Framework

Belladonna Play’s architecture is cross-platform by design. The Windows path follows the same contracts and behavior as Linux:

- **Unified HAL contracts:** the Windows backend implements the same interfaces for process hardening, capability reporting, and monitoring.  
- **User-mode first:** no kernel drivers. I prioritize least-privilege designs and measurable, observable behavior.  
- **Parity of features:** DRM and anti-cheat building blocks (entitlements, asset protection, mod signing/verification and allowlists) are OS-agnostic and will be wired through the Windows backend.  
- **Integration:** the upcoming Unreal/Unity/Godot plugins and SDKs will expose a consistent API surface across platforms.  
- **Connectivity:** no always-online requirement. Online verification and telemetry are opt-in and recommended primarily for live-service titles.  

Windows builds will follow once the backend stabilizes and passes performance and compatibility validation.  

---

## Security and Privacy Commitments

- **Least privilege by default.** Escalations only when necessary, with clear documentation.  
- **Transparency over obscurity.** Clear surfaces, predictable behavior, and measurable signals.  
- **Data minimization.** Collect only what I must to uphold integrity and safety goals.  
- **Respect for players.** Safe, nearly invisible for legitimate players, but hostile and unforgiving to attackers.  
- **No always-online requirement.** Online verification and telemetry are opt-in and developer-configurable; recommended for live-service titles only.  

---

## Releases

- **Target:** Linux x86_64 (glibc ≥ 2.31; kernel ≥ 5.10 recommended).  
- Verify artifacts using the accompanying SHA-256 file before distribution.  

---

## Getting Updates and Contact

- I’ll post updates here as the HAL/tooling open-source plan progresses and as engine plugins land.  
- For collaboration or evaluation inquiries, please contact the maintainers through this repository.

