# WJTTC Testing Strategy

## Philosophy
**F1-Inspired**: When brakes must work flawlessly, so must our code.

WJTTC is a **delivery validation framework** that complements language-specific unit test frameworks. Use BOTH for championship coverage.

---

## MANDATORY: Partner Testing Pairs

**RULE**: Every project MUST use a testing pair. WJTTC alone is not sufficient. Always run the language-specific unit test framework alongside WJTTC and consolidate the scores.

### Required Pairs

| Project Type | Unit Test Partner | + WJTTC | Combined Report |
|--------------|-------------------|---------|-----------------|
| Rust | `cargo test` | `run-all.sh` | REQUIRED |
| TypeScript | Vitest / Jest | `run-all.sh` | REQUIRED |
| Python | pytest | `run-all.sh` | REQUIRED |
| Go | `go test` | `run-all.sh` | REQUIRED |

### How to Record

Every test run MUST output a consolidated score:

```
=====================================
WJTTC COMBINED TEST REPORT
=====================================
Partner Framework: cargo test
Partner Tests:     30 passed (0.00s)
WJTTC Tests:       34 passed (0.57s)
-------------------------------------
COMBINED TOTAL:    64 passed
COMBINED TIME:     570ms
STATUS:            CHAMPIONSHIP GRADE
=====================================
```

### Never Accept

- WJTTC alone without partner tests
- Partner tests alone without WJTTC
- Uncombined/separate reporting

---

## Testing Stack by Language

### Rust Projects (like faf-wasm-sdk)
| Layer | Tool | Purpose |
|-------|------|---------|
| **Unit Tests** | `cargo test` | Function-level assertions |
| **Benchmarks** | `cargo bench` | Performance regression detection |
| **Linting** | `cargo clippy` | Code quality checks |
| **Delivery** | WJTTC | Binary exists, size acceptable, exports work |

```bash
# Full Rust testing
cargo test           # 30 unit tests
cargo clippy         # Lint checks
./wjttc/run-all.sh   # 34 delivery checks
```

---

### TypeScript/JavaScript Projects
| Layer | Tool | Purpose |
|-------|------|---------|
| **Unit Tests** | Vitest / Jest | Function-level assertions |
| **Type Checking** | `tsc --noEmit` | TypeScript validation |
| **Linting** | ESLint | Code quality |
| **E2E** | Playwright / Cypress | Browser automation |
| **Delivery** | WJTTC | Package.json valid, build works, exports correct |

```bash
# Full TS testing
npm test             # Vitest/Jest
npx tsc --noEmit     # Type check
npx eslint .         # Lint
./wjttc/run-all.sh   # Delivery checks
```

**Recommended**: Vitest (faster, ESM-native) over Jest for new projects.

---

### Python Projects
| Layer | Tool | Purpose |
|-------|------|---------|
| **Unit Tests** | pytest | Function-level assertions |
| **Benchmarks** | pytest-benchmark | Performance tracking |
| **Type Checking** | mypy | Static type analysis |
| **Linting** | ruff / flake8 | Code quality |
| **Delivery** | WJTTC | Module imports, CLI works, package valid |

```bash
# Full Python testing
pytest               # Unit tests
mypy .               # Type check
ruff check .         # Lint (fast)
./wjttc/run-all.sh   # Delivery checks
```

---

### Go Projects
| Layer | Tool | Purpose |
|-------|------|---------|
| **Unit Tests** | `go test` | Function-level assertions |
| **Benchmarks** | `go test -bench` | Performance tracking |
| **Linting** | golangci-lint | Code quality |
| **Delivery** | WJTTC | Binary builds, size check, runs correctly |

```bash
# Full Go testing
go test ./...              # Unit tests
go test -bench=. ./...     # Benchmarks
golangci-lint run          # Lint
./wjttc/run-all.sh         # Delivery checks
```

---

### Shell/Bash Scripts
| Layer | Tool | Purpose |
|-------|------|---------|
| **Syntax** | `bash -n` | Parse without execute |
| **Linting** | shellcheck | Best practices |
| **Delivery** | WJTTC | Scripts executable, correct shebang |

```bash
# Full Shell testing
shellcheck *.sh      # Lint
./wjttc/run-all.sh   # Delivery checks
```

---

## WJTTC Universal Checks

These apply to ANY project regardless of language:

| Check | What It Validates |
|-------|-------------------|
| **File existence** | Required files present |
| **Binary size** | Under threshold (e.g., <300KB WASM) |
| **Exports** | Public API correct |
| **Demo works** | Examples functional |
| **Config valid** | package.json, Cargo.toml, etc. parse correctly |
| **Performance** | Operations within time budget |

---

## Framework Selection Guide

| Scenario | Primary Tool | Why |
|----------|--------------|-----|
| Rust library | cargo test | Native, zero deps |
| Rust WASM | cargo test + wasm-pack test | Both native and browser |
| TypeScript new project | Vitest | Fastest, ESM-native |
| TypeScript existing Jest | Jest | Don't migrate unless needed |
| Python API | pytest | Most flexible |
| Python CLI | pytest + click.testing | CLI test utilities |
| Go microservice | go test | Native, no deps |
| Any delivery | WJTTC | Universal validation |

---

## Performance Testing Tools

| Tool | Language | Purpose |
|------|----------|---------|
| `cargo bench` | Rust | Criterion-based benchmarks |
| `hyperfine` | Any CLI | Command-line benchmarking |
| `k6` | HTTP | Load testing APIs |
| `pytest-benchmark` | Python | Function benchmarks |
| Vitest bench | TypeScript | Built-in benchmarking |

---

## CI/CD Integration

```yaml
# Example GitHub Actions
jobs:
  test:
    steps:
      # Language-specific
      - run: cargo test        # Rust
      - run: npm test          # TypeScript
      - run: pytest            # Python

      # Universal delivery validation
      - run: ./wjttc/run-all.sh
```

---

## Recording Test Results

Always capture and display:
1. **Test count**: How many tests ran
2. **Pass/fail ratio**: Green/red summary
3. **Time**: How long tests took
4. **Categories**: What was tested

Example output format:
```
================================
Cargo Tests:  30 passed (0.00s)
WJTTC Tests:  34 passed (0.57s)
--------------------------------
Total:        64 passed
Time:         570ms
Status:       CHAMPIONSHIP GRADE
================================
```

---

## Quick Reference

| Need | Use |
|------|-----|
| Test a Rust function | `cargo test` |
| Test a TypeScript function | Vitest / Jest |
| Test a Python function | pytest |
| Test API endpoints | pytest + httpx / supertest |
| Test browser behavior | Playwright / Cypress |
| Validate a deliverable | WJTTC |
| Benchmark performance | hyperfine / cargo bench |
| Load test an API | k6 |

---

*WJTTC - Wolfejam Testing and Test Center*
*Championship Grade Testing Since 2025*
