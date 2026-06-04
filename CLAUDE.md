<!-- faf: faf-wasm-sdk | Rust | sdk | The compiler is the spec. 322KB of WASM. No server. No API calls. No dependencies. -->
<!-- faf: claim=project.faf | family=FAF -->

# CLAUDE.md — faf-wasm-sdk

## What This Is

The compiler is the spec. 322KB of WASM. No server. No API calls. No dependencies.

## Stack

- **Language:** Rust
- **Hosting:** npm
- **Build:** wasm-pack
- **Cicd:** GitHub Actions

## Context

- **Who:** wolfejam
- **What:** WASM scoring kernel — the GCC of project DNA
- **Why:** One source of truth — same Rust code scores in CLI, browser, edge, and Node. No reimplementation, no drift.
- **Where:** npm (faf-wasm-sdk), embedded in builder.faf.one + Cloudflare Workers
- **When:** Nov 2025 (v1), Mar 2026 (v2 Definitive)
- **How:** Rust compiled to WASM via wasm-pack; 8 pure-function exports, JSON in/out

---

*STATUS: BI-SYNC ACTIVE — 2026-06-04T05:19:23.522Z*
