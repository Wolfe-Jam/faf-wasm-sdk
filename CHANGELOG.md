# Changelog

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
