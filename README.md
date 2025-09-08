# Belladonna Play (Fork Scaffold)

Game-focused fork built atop the Privilege Bridge core (now part of a Cargo workspace). This scaffold provides:

* Re-export of core crate (`olivine_privilege_bridge`)
* Early HAL abstraction (implemented in core) for cross-platform portability
* Placeholder modules: DRM, Anti-Cheat, SDK convenience layer

Next steps (aligns with roadmap Part 0–2):

1. Flesh out HAL: real seccomp + eBPF wiring (Linux), design Windows backend interfaces.
2. Replace direct OS calls in core with HAL indirection (incremental, guarded by feature flag).
3. Implement entitlement cache format + signature verification.
4. Introduce syscall / module integrity probes feeding anti-cheat stub.
5. Publish minimal engine integration sample calling `sdk::init()` + `sdk::check_entitlement()`.

This crate intentionally returns Unimplemented errors for now—focus is on establishing stable API seams.

## Workspace Usage

From repository root (`privilege-bridge/`):

```
cargo build --workspace
cargo test --workspace
```

Experimental HAL routing (incremental OS abstraction):

```
cargo test --features hal_unified --workspace
```

Only build/play with fork crate:

```
cargo build -p belladonna-play
```

## Play CLI (dev)

The fork includes a lightweight developer CLI:

```
cargo run -p belladonna-play --bin belladonna-play-cli -- status
cargo run -p belladonna-play --bin belladonna-play-cli -- hal-report --json
cargo run -p belladonna-play --bin belladonna-play-cli -- sysmon-bench --iters 10000 --baseline-out .bench/sysmon.json
```

These commands use the core HAL to inspect sandbox capabilities and measure SyscallMonitor overhead.

### Windows scaffolding

Initial Windows seams live in the core HAL under `src/hal/mod.rs` (cfg(windows)):

- SID mapping stubs (`current_sid_string`, `uid_to_sid`)
- Job Object design (+ `JobConfig` and setup stub)
- ETW session skeleton (used by `WinSys` SyscallMonitor)
- Module integrity enumeration stub
- WFP rules struct and apply stub

When developing on Windows, implement these stubs using Win32 APIs to achieve parity with Linux.
