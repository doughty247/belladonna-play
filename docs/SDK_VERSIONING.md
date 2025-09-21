# SDK Versioning & Stability

Status: Pre-0.1.0 (experimental). Breaking changes MAY occur with minor bumps until 0.1.0.

## SemVer Policy Roadmap
- 0.0.x: Rapid iteration, refine API naming & ergonomics.
- 0.1.0: FFI surface frozen (additive only afterward).
- 0.2.x: Add metrics & event reporting hooks.
- 1.0.0: Post engine plugin adoption + production validation.

## Compatibility Guarantees (Current)
- No silent behavior changes without version increment.
- API removals documented in CHANGELOG (to be added).

## FFI Forward Compatibility (Planned)
- ABI version integer.
- Reserved padding fields in public structs for extension.
- Error codes stable after 0.1.0.
