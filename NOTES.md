# Notes — Belladonna Play

Operating assumptions
- Play builds atop Privilege Bridge core (workspace member).
- Linux-first development, Windows parity stubs in HAL until environment switch.
- Observability: Prometheus + small HTTP helpers; keep default overhead low.

Key snippets
- HAL snapshot overhead target: p50 < 5µs, p99 < 50µs.
- Capability gauges: seccomp_loaded, landlock_present, namespaces_active.
- Sysmon events total: computed via snapshot delta in Prometheus exporter.

Open questions
- DRM entitlement cache sealing approach (TPM vs OS DPAPI vs app-keyed AEAD)?
- ETW provider set for syscall coverage on Windows (kernel vs user channels)?
- WFP policy granularity (domain allow-list vs process-bound rules)?

Handy commands
- Build Play CLI only: `cargo build -p belladonna-play --bins`
- HAL report: `belladonna-play-cli hal-report --json`
- eBPF detect: `belladonna-play-cli ebpf-detect --json`
- Benchmark baseline: `belladonna-play-cli sysmon-bench --iters 10000 --baseline-out .bench/sysmon.json`