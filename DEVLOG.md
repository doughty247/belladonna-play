# Dev Log — Belladonna Play

Purpose: running, plain-text progress so context survives across machines (Linux ↔ Windows).

## 2025-09-08

Highlights
- Play CLI added: `hal-report`, `sysmon-snapshot`, `sysmon-bench --baseline-out`, `ebpf-detect`.
- HAL docs expanded in `WORKSPACE.md` (detailed contract, perf budgets).
- Prometheus exporter wired with HAL capability gauges; sysmon delta counter.
- Windows scaffolding added (stubs): SID mapping, Job Object notes, ETW session skeleton, module integrity, WFP rules.

Next
- P0.15: finalize capability JSON + metrics polish (schema note, doc link).
- P0.18: advance eBPF SyscallMonitor under feature flag; keep perf fallback.
- Part 2 (DRM): entitlement check stub + encrypted offline cache type; Play CLI `check-entitlement`.

Notes
- Baseline capture example:
  - `belladonna-play-cli sysmon-bench --iters 10000 --baseline-out .bench/sysmon.json`
- Capability report quick check:
  - `belladonna-play-cli hal-report --json`
