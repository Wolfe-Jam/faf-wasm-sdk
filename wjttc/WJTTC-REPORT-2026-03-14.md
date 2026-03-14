---
# WJTTC Test Report
project: "faf-wasm-sdk"
version: "2.0.0"
feature: "Mk4 Championship Engine + FAFb Binary Format"
tester: "wolfejam (via Claude)"
date: "2026-03-14"
tier: "Tier 1 - Brake Systems + Tier 2 - Engine Systems"
---

## Test Summary

**Objective:** Validate the v2.0.0 rewrite — Mk4 33-slot scoring engine, FAFb binary compiler, and pure-function WASM API under normal, edge-case, and adversarial conditions.

**Result:** PASS
**Duration:** <1s (136 tests in 0.22s)
**Environment:** macOS Darwin 22.6.0, Rust 1.94.0 (rustup), faf-rust-sdk 1.3.0

## Test Statistics

- **Total Tests:** 136
- **Passed:** 136
- **Failed:** 0
- **Blocked:** 0
- **Pass Rate:** 100%

## Test Breakdown

| Module | Tests | Tier | Coverage |
|--------|-------|------|----------|
| mk4::tests | 52 | Tier 2 Engine | Slot states, placeholders, tiers, parity, edge cases |
| fafb::tests | 28 | Tier 1 Brake | Compile, decompile, roundtrip, checksum, unicode |
| lib::tests | 9 | Tier 2 Engine | Public API surface integration |
| tests/wasm.rs | 21 | Tier 2 Engine | External integration via inner modules |
| tests/stress.rs | 27 | Tier 1 Brake | Adversarial inputs, crash resistance |

## Tier 1: Brake Systems (Life-Critical)

### Binary Format Integrity

| Test | Input | Expected | Actual | Status |
|------|-------|----------|--------|--------|
| FAFB magic bytes | Valid YAML | bytes[0..4] == "FAFB" | "FAFB" | PASS |
| CRC32 consistency | Same YAML x2 | Identical checksums | Identical | PASS |
| CRC32 uniqueness | Different YAML | Different checksums | Different | PASS |
| Compile/decompile roundtrip | Valid YAML | Content preserved | Preserved | PASS |
| Unicode roundtrip | Japanese/Arabic/math | No corruption | Clean | PASS |
| Empty input | "" | Clean error | "Source content is empty" | PASS |
| Invalid YAML | Broken syntax | Clean error | Parse error message | PASS |
| Truncated header | 4 bytes | Clean error | Error (not panic) | PASS |
| Random bytes | 0xFF x100 | Clean error | Error (not panic) | PASS |
| Zero bytes | 0x00 x100 | Clean error | Error (not panic) | PASS |
| One byte | 0x46 | Clean error | Error (not panic) | PASS |
| Garbage with FAFB magic | FAFB + 0xFF | Clean error | Error (not panic) | PASS |
| 50KB YAML | 1000 tech stack items | Compile + roundtrip | Clean | PASS |

### Divide-by-Zero Protection

| Test | Scenario | Expected | Actual | Status |
|------|----------|----------|--------|--------|
| All 21 slotignored | active=0, populated=0 | score=0 | 0 | PASS |
| JSON output | All ignored | No NaN/Infinity | Clean JSON | PASS |

### Crash Resistance (Adversarial Inputs)

| Test | Attack Vector | Result | Status |
|------|---------------|--------|--------|
| YAML bomb (50 levels deep) | Stack overflow | Handled, score=0 | PASS |
| YAML bomb (1000 keys) | Memory pressure | Handled, score=0 | PASS |
| 100KB slot value | Large allocation | Populated correctly | PASS |
| Unicode slot names | Non-ASCII keys | Graceful (no match) | PASS |
| Emoji values | Multi-byte chars | Populated correctly | PASS |
| Binary/null bytes | Control chars | No crash | PASS |
| Duplicate YAML keys | Strict parse | Clean error with "duplicate" | PASS |
| Tab indentation | Invalid YAML | Clean error | PASS |
| Numeric keys | Non-string keys | Graceful (no match) | PASS |
| Boolean keys | true/false as keys | Graceful (no match) | PASS |
| YAML anchors/aliases | Merge keys | No crash | PASS |
| Multiline strings | Block/folded scalars | Populated correctly | PASS |
| Null byte validation | "\0" | false (not crash) | PASS |
| Whitespace-only validation | Spaces/tabs/newlines | false (not crash) | PASS |
| Control chars validation | 0x00-0x1F | No crash | PASS |
| 10K-line YAML validation | 10,000 unique keys | true (valid mapping) | PASS |

## Tier 2: Engine Systems (Accuracy)

### Mk4 Scoring Accuracy

| Slots | Expected Score | Actual | Tier | Status |
|-------|---------------|--------|------|--------|
| 0/21 | 0% | 0 | Red | PASS |
| 1/21 | 5% | 5 | Red | PASS |
| 3/21 | 14% | 14 | Red | PASS |
| 6/21 | 29% | 29 | Red | PASS |
| 11/21 | 52% | 52 | Red | PASS |
| 17/17 (4 ignored) | 100% | 100 | Trophy | PASS |
| 21/21 | 100% | 100 | Trophy | PASS |
| 33/33 (Enterprise) | 100% | 100 | Trophy | PASS |
| 24/33 (Mixed) | 73% | 73 | Green | PASS |

### Tier Boundary Accuracy

| Score | Expected Tier | Actual | Status |
|-------|---------------|--------|--------|
| 100 | Trophy | Trophy | PASS |
| 99 | Gold | Gold | PASS |
| 98 | Silver | Silver | PASS |
| 95 | Silver | Silver | PASS |
| 94 | Bronze | Bronze | PASS |
| 85 | Bronze | Bronze | PASS |
| 84 | Green | Green | PASS |
| 70 | Green | Green | PASS |
| 69 | Yellow | Yellow | PASS |
| 55 | Yellow | Yellow | PASS |
| 54 | Red | Red | PASS |
| 0 | Red | Red | PASS |

### Placeholder Rejection

| Input | Expected | Actual | Status |
|-------|----------|--------|--------|
| "describe your project goal" | Rejected | Rejected | PASS |
| "development teams" | Rejected | Rejected | PASS |
| "cloud platform" | Rejected | Rejected | PASS |
| "null" | Rejected | Rejected | PASS |
| "none" | Rejected | Rejected | PASS |
| "unknown" | Rejected | Rejected | PASS |
| "n/a" | Rejected | Rejected | PASS |
| "not applicable" | Rejected | Rejected | PASS |
| "NULL" (case) | Rejected | Rejected | PASS |
| "  null  " (whitespace) | Rejected | Rejected | PASS |
| "" (empty) | Rejected | Rejected | PASS |
| "  " (whitespace-only) | Rejected | Rejected | PASS |
| YAML bare null | Rejected (Value::Null) | Rejected | PASS |
| YAML tilde ~ | Rejected (Value::Null) | Rejected | PASS |

### Slotignored Behavior

| Test | Expected | Actual | Status |
|------|----------|--------|--------|
| "slotignored" (lowercase) | Excluded from denominator | Excluded | PASS |
| "Slotignored" (capitalized) | Treated as valid string | Populated | PASS |
| "SLOTIGNORED" (uppercase) | Treated as valid string | Populated | PASS |

### Determinism

| Test | Expected | Actual | Status |
|------|----------|--------|--------|
| Same YAML scored 3x | Identical scores | Identical | PASS |
| Base vs Enterprise (same data) | Base% > Enterprise% | 14% > 9% | PASS |

## Bugs Found During Testing

### Bug #1: Duplicate YAML Keys (Test Expectation)
**Severity:** None (test error, not engine bug)
**Details:** serde_yaml 0.9 strictly rejects duplicate keys per YAML 1.2 spec. Test assumed "last value wins" behavior. Engine correctly returns clean error. Test fixed.

### Bug #2: 10K Duplicate Keys (Test Expectation)
**Severity:** None (test error, not engine bug)
**Details:** `"a: b\n".repeat(10_000)` creates 10K duplicate `a` keys. Same duplicate rejection. Test fixed to use unique keys.

### Bug #3: Clippy Warnings on Rust 1.94.0
**Severity:** Low (code quality)
**Details:** 5 new clippy warnings from upgrading 1.91.1 -> 1.94.0. `needless_borrows_for_generic_args` (4) and `match_like_matches_macro` (1). All fixed.

## Performance

| Operation | Time | Status |
|-----------|------|--------|
| 136 tests total | 0.22s | PASS |
| Mk4 scoring (single) | <1ms | PASS |
| FAFb compile (small) | <1ms | PASS |
| FAFb compile (50KB) | <5ms | PASS |
| WASM binary size | 281KB | PASS |

## Code Quality

| Check | Status |
|-------|--------|
| cargo clippy (zero warnings) | PASS |
| cargo test (136/136) | PASS |
| No unsafe code | PASS |
| No panics in production paths | PASS |
| All errors are Result<_, String> | PASS |

## Architecture Validation

| Check | Status |
|-------|--------|
| 3 source files only (lib.rs, mk4.rs, fafb.rs) | PASS |
| Old v1.x files removed (5 files, 2,251 lines) | PASS |
| 8 pure-function exports (no classes) | PASS |
| No FAF class in WASM output | PASS |
| faf-rust-sdk 1.3 dependency (FAFb) | PASS |
| Base (21 slots) and Enterprise (33 slots) | PASS |

## Championship Certification

**Status: CHAMPIONSHIP GRADE**

| Criteria | Result |
|----------|--------|
| Pass Rate | 100% (136/136) |
| Clippy Warnings | 0 |
| Tier 1 Brake Tests | 27/27 PASS |
| Tier 2 Engine Tests | 109/109 PASS |
| Adversarial Resistance | No crashes, no panics, no corruption |
| Binary Integrity | CRC32 verified, roundtrip clean |
| WASM Size | 281KB (under 300KB target) |

---

*Tested with WJTTC Championship Standards*
*"We break things so others never have to know they were broken."*
*faf-wasm-sdk v2.0.0 — Brakes work at 220mph.*
