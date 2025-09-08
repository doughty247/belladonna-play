Belladonna Fork Roadmap  
From Privilege Bridge to DRM + Anti-Cheat

### Preface — Why Fork Instead of Replace

Belladonna began as a Privilege Bridge: a hardened system for managing replay protection, sandboxing, eBPF monitoring, deception, audit logs, and policy governance. That core is already valuable in its own right, especially for enterprises and security-conscious services.

But in games, the biggest problems are piracy and cheating. These cost studios billions in lost revenue and player churn. Belladonna’s foundation makes it uniquely positioned to solve these problems — if adapted into a purpose-built platform.

We fork, we don’t replace:

* Belladonna Core (Privilege Bridge): general-purpose secure foundation.
* Belladonna Play: game-focused fork (HAL, DRM entitlements, runtime anti-cheat, engine plugins, player trust).

Analogy: Core = kernel. Play = gaming distribution built on that kernel. This avoids bloat and maximizes reuse.

---
## Global Progress Checklist
Legend: [ ] pending  [~] in progress  [x] done (initial implementation)  
Refine statuses once execution begins.

| Part | Title | Status |
|------|-------|--------|
| 0 | HAL (cross‑platform core) | [~] |
| 1 | Security Spine Packaging | [ ] |
| 2 | DRM Enablement | [ ] |
| 3 | Runtime Anti‑Cheat | [ ] |
| 4 | SDKs & Engine Plugins | [ ] |
| 5 | Observability & Ops | [ ] |
| 6 | Machine Learning (Advisory) | [ ] |
| 7 | Player Trust & Fairness | [ ] |
| 8 | Business / Market Fit | [ ] |

---
## Part 0 — Build the HAL (Hardware Abstraction Layer)
Context: Unified abstraction for Linux + Windows primitives (sandboxing, syscalls, privilege, timing) to prevent platform drift.

NOTE: Active development machine = Linux. Windows-specific logic will be implemented later when switching environments; placeholders and structural seams are in place now.

Unified Core Checklist:
- [x] Enumerate responsibilities (sandboxing, syscall monitor, privilege, timing, integrity sampling)
- [x] Define core traits (`ProcessGuard`, `SyscallMonitor`, `SandboxManager`)
- [x] Add feature flag `hal_unified`
- [x] Initial HAL module + smoke test
- [x] Workspace layout supporting multi-crate fork
- [~] HAL contract documentation (error modes, safety, perf goals)
- [x] CLI hal-report command
- [x] Deterministic capability reporting fields (seccomp_loaded, landlock_enabled, namespaces_active)
- [~] Anti-debug abstraction (remove direct ptrace usage from hydra)

Linux-Specific Checklist:
- [x] Skeleton Linux backend (uid + sandbox stub + sysmon stub)
- [x] Route `getuid` through HAL under feature flag
- [x] Wire basic + seccomp sandbox via HAL
- [~] Add Landlock status detection
- [x] Namespace detection (pid, net, mount)
- [~] eBPF probe integration for SyscallMonitor (HAL loader scaffold added; ringbuf -> counter; real probes next)
- [x] Replace ptrace anti-debug check via HAL layer
- [x] Network tightening placeholder (`tighten_network` impl)
- [x] Filesystem restriction prototype (`restrict_filesystem` selective allow)
- [~] Performance overhead micro-bench (<1% target) baseline capture

Windows-Specific Checklist (Framework Only Until Environment Switch):
- [x] Windows backend stub module (process, sandbox, sysmon)
- [x] UID / SID mapping function (stub API seams in HAL windows module)
- [x] Job Object sandbox integration design notes (JobConfig + setup stub)
- [x] ETW session setup scaffold (EtwSession + WinSys)
- [x] DLL/module integrity enumeration stub (ModuleInfo + enumerate stub)
- [x] Network restriction strategy outline (WFP rules struct + apply stub)
- [ ] Future: parity test harness run on Windows host

Cross-Platform / Integration Checklist:
- [~] Cross-platform test harness (Linux done, Windows pending) 
- [~] Replace direct OS calls incrementally (uid done; ptrace/prctl pending) 
- [~] Capability JSON export for observability — added optional capability HTTP endpoint and CLI hal-report
- [~] Metrics: expose HAL capability counters (gauge: seccomp_on, landlock_on) — sysmon events counter wired; capability gauges (seccomp_loaded, landlock_present, namespaces_active) added; Prometheus polls HAL snapshots
- [x] Documentation page in `WORKSPACE.md` linking HAL concepts
- [x] Play CLI: added `belladonna-play-cli` with `hal-report`, `sysmon-snapshot`, `sysmon-bench --baseline-out`


Refined Incremental (P0.x) Tracker:
| ID | Task | Status | Notes |
|----|------|--------|-------|
| P0.1 | HAL module + traits | [x] | Done |
| P0.2 | Feature flag `hal_unified` | [x] | Done |
| P0.3 | Linux skeleton backend | [x] | Done |
| P0.4 | Windows stub backend | [x] | Done |
| P0.5 | Route getuid | [x] | Done |
| P0.6 | Seccomp via HAL | [x] | Done (basic) |
| P0.7 | Smoke test | [x] | Done |
| P0.8 | Expand SandboxReport fields | [x] | Done |
| P0.9 | Deterministic seccomp_loaded flag | [x] | Done |
| P0.10 | Landlock detection | [~] | Kernel probe added; enforcement detection next |
| P0.11 | Namespace detection | [x] | Done |
| P0.12 | CLI hal-report command | [x] | Done |
| P0.13 | Anti-debug abstraction | [~] | HAL probe wired; hydra fallback remains |
| P0.14 | Additional libc call routing | [ ] | Pending |
| P0.15 | Capability JSON export + metrics | [~] | In progress |
| P0.16 | HAL docs in WORKSPACE / contract section | [x] | Detailed contract, error modes, perf budgets |
| P0.17 | Performance overhead benchmark | [x] | Baseline recorder via --baseline-out |
| P0.18 | eBPF SyscallMonitor integration | [~] | HAL loader scaffold; prefer eBPF -> perf fallback; Prometheus counter wired; CLI dev sysmon-snapshot added; Play CLI ebpf-detect added |
| P0.19 | Windows design notes (Job Objects, ETW) | [ ] | Later |
| P0.20 | Cross-platform parity harness (Win) | [ ] | Later |

Current Focus: P0.15, P0.16, P0.17, P0.18 (capability JSON + metrics, HAL contract docs, perf baseline, eBPF probe integration) on Linux.

## Part 1 — Foundation and Goal
Context: Reuse existing sandboxing, replay/nonce guard, Hydra/Haze deception, tamper-evident audit. Present as unified “security spine”.

Checklist:
- [ ] Validate sandbox via HAL substitution
- [ ] Load / stress replay guard under concurrency
- [ ] Exercise Hydra/Haze decoys in staging builds
- [ ] Prove audit immutability (chain verification & Merkle anchors)
- [ ] Position fork messaging: "one platform for DRM + anti‑cheat"

## Part 2 — DRM Enablement
Context: Invisible when valid; decisive when invalid. Support offline grace window.

Checklist:
- [ ] Entitlement check at launch (remote or cached)
- [ ] Encrypted local entitlement cache (sealed / monotonic expiry)
- [ ] Configurable offline play window (7–30 days)
- [ ] Build‑time encryption of binaries/assets
- [ ] Runtime decrypt with Belladonna key service (no plaintext at rest)
- [ ] Mod signing & whitelist (hash + signature)
- [ ] License models: subscription, rental, perpetual
- [ ] User‑facing failure messaging (localized, non hostile)

## Part 3 — Runtime Anti-Cheat
Context: Low overhead anomaly & integrity detection with deception triggers.

Checklist:
- [ ] File hash verification (executables + critical DLLs / SOs)
- [ ] Module whitelist enforcement (deny unexpected injections)
- [ ] Syscall/ETW monitoring (Linux eBPF, Windows ETW channels)
- [ ] Session token binding per network packet
- [ ] Periodic session token rotation (entropy + replay resistance)
- [ ] Per‑session packet encoding randomization
- [ ] Input capture for timing / physics anomaly analysis
- [ ] Baseline model for “impossible movement” & rate thresholds
- [ ] Decoy trigger pipeline (inject deceptive artifacts)
- [ ] Decoy hit fingerprint logging (cheater clustering)

## Part 4 — Developer Integration (SDKs & Plugins)
Context: Frictionless adoption across major engines.

Checklist:
- [ ] Core SDK crate (Rust)
- [ ] Stable FFI (C / C++) surface
- [ ] Unity plugin (C# wrapper + inspector tooling)
- [ ] Unreal plugin (Blueprint nodes + C++ hooks)
- [ ] Godot GDExtension module (godot-rust)
- [ ] Core API: `belladonna_init`, `check_entitlement`, `report_event`, `status`
- [ ] Server APIs: session validation & anomaly reporting
- [ ] CLI augmentation: config linting, policy diff reuse
- [ ] Publish engine sample projects (Unity/Unreal/Godot)
- [ ] Quickstart + per‑engine integration docs

## Part 5 — Observability & Operations
Context: Give studios immediate insight & operational confidence.

Checklist:
- [ ] Prometheus metrics for DRM + anti‑cheat domains
- [ ] Packaged Grafana dashboards (replay rates, decoys, latency, anomalies)
- [ ] Default alert rules (latency, cheat anomaly thresholds, entitlement failures)
- [ ] SIEM connectors (Splunk / Elastic / Datadog) export adapters
- [ ] Chat / on‑call hooks (Slack, PagerDuty, Teams)
- [ ] Pilot SOC starter kit (configs + runbooks)
- [ ] Incident runbooks (sandbox divergence, entitlement outage, probe failure)

## Part 6 — Machine Learning (Future Phase)
Context: Begin advisory mode only; no auto‑ban until validated.

Checklist:
- [ ] Telemetry collection gating & privacy review
- [ ] Feature stores (syscalls, behavior metrics, decoy interactions)
- [ ] Offline model training on replayed historical sessions
- [ ] Chaos/fuzz validation of model stability
- [ ] Advisory‑only ML scoring channel
- [ ] Compare predictions vs. manual review set
- [ ] Graduated enablement path for limited auto‑escalation

## Part 7 — Player Trust & Fairness
Context: Transparency and proportional enforcement.

Checklist:
- [ ] Publish plain‑language "Fair Play" document
- [ ] Appeal workflow linking audit log evidence
- [ ] Graduated enforcement ladder (warn → monitor → escalate)
- [ ] Guaranteed offline play inside entitlement window
- [ ] Collect only non‑identifiable system attributes (document list)
- [ ] Clear player messages for entitlement failures (retry + support path)

## Part 8 — Business Pitch & Market Fit
Context: Dual value: protect revenue + reduce churn.

Checklist:
- [ ] Positioning narrative: "immune system for games"
- [ ] Open‑core / indie tier definition
- [ ] Mid‑tier pricing model (0.10–0.20 CAD/user/month)
- [ ] AAA enterprise contract structure (10–30M CAD/year target)
- [ ] Market sizing math (3% share ≈ 9B CAD/year) validation
- [ ] ROI collateral: refund reduction & churn metrics
- [ ] Branding: dual‑purpose DRM + anti‑cheat campaign assets

---
Narrative Tagline: Brand Belladonna as dual-purpose DRM + anti-cheat.