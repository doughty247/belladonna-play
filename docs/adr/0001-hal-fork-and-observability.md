# ADR 0001: HAL Fork Strategy and Observability

Date: 2025-09-08

Context
- Belladonna Play is a fork focused on DRM + anti-cheat built atop Privilege Bridge core.
- We need cross-platform portability (Linux + Windows) with minimal drift.
- Operators require low-overhead observability for trust & debugging.

Decision
- Keep HAL in the core crate; Play reuses it directly.
- Add cfg(windows) stubs (SID mapping, Job Object notes, ETW skeleton, module integrity, WFP outline).
- Expose lightweight capability JSON and Prometheus metrics; snapshot-based sysmon deltas.

Consequences
- Faster initial progress; Windows specifics can be implemented later without breaking Play APIs.
- Clear seams reduce risk when switching dev environment.
- Metrics/HTTP helpers allow quick validation without heavy infra.
