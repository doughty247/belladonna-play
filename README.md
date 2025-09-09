# Belladonna Play

One platform for DRM + anti‑cheat — built on Belladonna Core.

Status: v0.0.1 Foundation (Linux‑only). DRM is ready; runtime anti‑cheat lands next.

Key features in v0.0.1:
- Encrypted entitlement cache (AES‑256‑GCM) with monotonic epoch
- Default‑infinite offline policy; opt‑in enforcement window
- Asset packer (.bdpack) and runtime decrypt (no plaintext at rest)
- Mod signing/verify (Ed25519) and SHA256 allowlist manifest
- HAL/sysmon tooling (eBPF scaffold or perf fallback) and capability reporting

Download
- Releases: https://github.com/doughty247/belladonna-play/releases/tag/v0.0.1
- Artifact: belladonna-play-0.0.1-linux-x86_64.tar.gz

Quickstart (binary)
```bash
tar -xzf belladonna-play-0.0.1-linux-x86_64.tar.gz
cd belladonna-play-0.0.1-linux-x86_64
./bin/belladonna-play-cli status
./bin/belladonna-play-cli hal-report --json
./bin/belladonna-play-cli sysmon-bench --iters 10000 --baseline-out .bench/sysmon.json
```

DRM essentials
```bash
# Seal an entitlement cache (perpetual example)
./bin/belladonna-play-cli seal-entitlement --user alice --entitled --expires "2030-01-01T00:00:00Z"

# Offline check (infinite by default); to enforce a window:
./bin/belladonna-play-cli offline-check --enforce --days 14
```

Assets and mods
```bash
# Pack and encrypt assets
./bin/belladonna-play-cli asset-pack --input ./game_assets --out dist/assets.bdpack

# Extract at runtime
./bin/belladonna-play-cli asset-extract --pack dist/assets.bdpack --out /tmp/assets

# Mod signing / verification / allowlist
./bin/belladonna-play-cli mod-sign --file my_mod.pak --sk <ED25519_SK_HEX>
./bin/belladonna-play-cli mod-verify --file my_mod.pak --pk <ED25519_PK_HEX>
./bin/belladonna-play-cli manifest-gen --dir ./mods --out mods.sha256
./bin/belladonna-play-cli mod-allow --file my_mod.pak --manifest mods.sha256
```

Build from source (dev)
```bash
cargo build -p belladonna-play
cargo test -p belladonna-play
```

Docs
- Roadmap: changelogs/Belladonna_Play_Roadmap.md
- HAL contract (Play): docs/HAL_CONTRACT.md
- Release notes: RELEASE_NOTES_v0.0.1.md

Notes
- Keys: core keymgmt reads base64 32‑byte HMAC key from `/etc/belladonna/keys/hmac.key`.
- Windows support is stubbed and will arrive in a later release.
