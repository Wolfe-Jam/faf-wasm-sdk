# Changelog

## Unreleased

### Added
- crates.io Trusted Publishing (OIDC) workflow (`publish-crate.yml`) — publishes
  on GitHub release via `rust-lang/crates-io-auth-action`, no long-lived token
- Weekly `cargo audit` security workflow (`audit.yml`) — RustSec advisories on
  push to Cargo.toml/Cargo.lock, weekly schedule, and manual dispatch

### Changed
- Rust edition 2021 → 2024; declared `rust-version = "1.85"` (MSRV)
- `cargo fmt` applied for the 2024 style edition (formatting only, no logic
  changes)
- wasm-bindgen 0.2.105 → 0.2.123 and wasm-bindgen-test 0.3.55 → 0.3.73
  (lockfile-only; manifest stays caret `0.2`)

## v2.1.0 — Mk4 canonical slot names (2026-05-14)

- Rename 6 stack slot identifiers to their Mk4 canonical names, with a
  backward-compat alias fallback so existing `.faf` files (legacy keys) keep
  scoring correctly (faf-cli #66 Phase B):
  - `stack.frontend` → `stack.framework`
  - `stack.css_framework` → `stack.css`
  - `stack.state_management` → `stack.state`
  - `stack.api_type` → `stack.api`
  - `stack.database` → `stack.db`
  - `stack.package_manager` → `stack.pkg_manager`

## v2.0.0 — The Definitive Edition (2026-03-19)

- **String table** — every YAML key becomes a named binary section (up to 256)
- **Chunk classification** — DNA (core identity), Context (supplementary), Pointer (doc refs)
- **Mk4 scoring engine** — 33-slot enterprise scoring, same engine as Rust SDK and CLI
- **Deterministic output** — same YAML in, same binary out, CRC32 sealed
- **faf-rust-sdk v2.0.0** — now pulls from crates.io (was path dependency)
- 138 tests across unit, stress, and WASM integration suites

## v1.2.1 (2026-02-04)

- Multi-language detection and type intelligence
- ML framework detection from README
- Bug fixes

## v1.2.0 (2026-02-03)

- Multi-language detection
- Type detection in WASM generator

## v1.0.0 (2026-01-15)

- Initial release
- Dual scoring engine (base + enterprise)
- FAFb compile/decompile
- 8 WASM exports
