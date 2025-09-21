# Belladonna SDK (Rust)

High-level, stable(ish) facade over `belladonna-play` for game integrations.

## Goals
- Minimize surface area exposed to engines & foreign language bindings.
- Provide lifecycle control (init, entitlement check, integrity orchestrator enable/disable).
- Centralize error taxonomy.
- Forward-compatible: additions are non-breaking via feature flags.

## Quickstart
```rust
use belladonna_sdk::{InitConfig, SdkHandle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = SdkHandle::init(InitConfig { auto_integrity: true })?;
    let ent = sdk.check_entitlement("player1");
    println!("entitled={}", ent.entitled);
    Ok(())
}
```

## Features
- `drm` (default): entitlement wrappers.
- `integrity` (default): integrity orchestrator control.
- `deception`: (placeholder) deception drill forwarding.
- `ffi`: enables C ABI layer (`bd_*` symbols) via `src/ffi.rs`.

## Thread Safety
- `SdkHandle` is `Send + Sync` (internals behind `parking_lot::Mutex`).
- Orchestrator start/stop is idempotent; repeated calls are cheap.

## Versioning
See `docs/SDK_VERSIONING.md` (semver; experimental until 0.1.0).

## Planned Additions
- Metrics export hook (`bd_export_metrics`) – reserved prototype in header.
- Logger callback registration (`bd_set_logger`).
- Structured entitlement result (error code + expiry timestamp).
- External runtime injection feature flag for host-owned Tokio.
