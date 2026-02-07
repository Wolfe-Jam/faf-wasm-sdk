# WJTTC Standard v1.0.0

## Wolfejam Testing and Test Center
**F1-Inspired**: When brakes must work flawlessly, so must our code.

---

## The Standard

Every FAF project MUST implement the WJTTC Partner Stack with:
- A language-specific **Partner Framework** (unit tests)
- The universal **WJTTC Delivery Validation** (integration/delivery tests)
- A **Combined Report** showing both scores

---

## Required Sections (6 Sections)

| Section | Name | Purpose |
|---------|------|---------|
| 1 | Build Tests | Files exist, artifacts valid, size checks |
| 2 | Lint Check | Code quality gate (clippy/eslint/ruff) |
| 3 | Unit Tests | Partner framework (cargo/vitest/pytest) |
| 4 | Logic Tests | Algorithm/business logic validation |
| 5 | API Tests | Exports, interfaces, public surface |
| 6 | Demo Tests | Examples work, docs accurate |

---

## Partner Framework by Language

| Language | Partner Framework | Lint Tool |
|----------|------------------|-----------|
| Rust | `cargo test` | `cargo clippy` |
| TypeScript | `vitest` / `jest` | `eslint` |
| Python | `pytest` | `ruff` |
| Go | `go test` | `golangci-lint` |

---

## Required Output Format

```
=====================================
WJTTC COMBINED TEST REPORT
=====================================
Partner Framework: {framework_name}
Partner Tests:     {count} passed ({time})
WJTTC Tests:       {count} passed, {fail} failed
-------------------------------------
COMBINED TOTAL:    {total} passed
COMBINED TIME:     {time}ms

STATUS: {CHAMPIONSHIP GRADE | X issues to fix}
=====================================
```

---

## Minimum Test Counts

| Project Size | Partner Tests | WJTTC Tests | Combined Minimum |
|--------------|---------------|-------------|------------------|
| Small (<1k LOC) | 10+ | 15+ | 25+ |
| Medium (1-5k LOC) | 25+ | 25+ | 50+ |
| Large (5k+ LOC) | 50+ | 35+ | 85+ |

FAF WASM SDK (medium): 30 + 34 = **64 tests** (exceeds minimum)

---

## File Structure

```
project/
├── wjttc/
│   ├── run-all.sh          # Main test runner (REQUIRED)
│   ├── stress-test.sh      # Performance stress test (OPTIONAL)
│   ├── WJTTC-STANDARD.md   # This file (REQUIRED)
│   └── TESTING-STRATEGY.md # Project-specific notes (OPTIONAL)
```

---

## run-all.sh Template

```bash
#!/bin/bash
# WJTTC - {PROJECT_NAME}
# F1-Inspired: When brakes must work flawlessly, so must our code

set +e

echo ""
echo "  WJTTC - {PROJECT_NAME} Test Suite"
echo "  ================================"
echo ""

PASS=0
FAIL=0
START=$(python3 -c "import time; print(int(time.time()*1000))")

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

pass() { echo -e "  ${GREEN}✓${NC} $1"; ((PASS++)); }
fail() { echo -e "  ${RED}✗${NC} $1"; ((FAIL++)); }

# ============================================
# SECTION 1: Build Tests
# ============================================
echo -e "${CYAN}[1/6] Build Tests${NC}"
# TODO: Add file existence checks
# TODO: Add artifact size checks

# ============================================
# SECTION 2: Lint Check
# ============================================
echo -e "${CYAN}[2/6] Lint Check${NC}"
# TODO: Add lint command for your language

# ============================================
# SECTION 3: Unit Tests (Partner Framework)
# ============================================
echo -e "${CYAN}[3/6] Unit Tests${NC}"
# TODO: Run partner framework and capture count

# ============================================
# SECTION 4: Logic Tests
# ============================================
echo -e "${CYAN}[4/6] Logic Tests${NC}"
# TODO: Validate business logic

# ============================================
# SECTION 5: API Tests
# ============================================
echo -e "${CYAN}[5/6] API Tests${NC}"
# TODO: Check exports and interfaces

# ============================================
# SECTION 6: Demo Tests
# ============================================
echo -e "${CYAN}[6/6] Demo Tests${NC}"
# TODO: Validate examples work

# ============================================
# COMBINED REPORT
# ============================================
END=$(python3 -c "import time; print(int(time.time()*1000))")
ELAPSED=$((END - START))

WJTTC_TESTS=$((PASS - 1))
PARTNER_TESTS=${TEST_COUNT:-0}
COMBINED_PASS=$((PARTNER_TESTS + WJTTC_TESTS))

echo ""
echo "  ====================================="
echo -e "  ${CYAN}WJTTC COMBINED TEST REPORT${NC}"
echo "  ====================================="
echo "  Partner Framework: {FRAMEWORK}"
echo -e "  Partner Tests:     ${GREEN}${PARTNER_TESTS} passed${NC}"
echo -e "  WJTTC Tests:       ${GREEN}${WJTTC_TESTS} passed${NC}, ${RED}${FAIL} failed${NC}"
echo "  -------------------------------------"
echo -e "  ${CYAN}COMBINED TOTAL:    ${COMBINED_PASS} passed${NC}"
echo "  COMBINED TIME:     ${ELAPSED}ms"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "  STATUS: ${GREEN}CHAMPIONSHIP GRADE${NC}"
else
    echo -e "  STATUS: ${RED}${FAIL} issues to fix${NC}"
fi
echo "  ====================================="
echo ""

exit $FAIL
```

---

## Quality Gates

| Gate | Requirement |
|------|-------------|
| Lint | Zero warnings/errors |
| Partner Tests | 100% passing |
| WJTTC Tests | 100% passing |
| Combined | Meets minimum for project size |

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Cold run | <60s |
| Warm run | <5s |
| Per-test | <100ms average |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-11-26 | Initial standard |

---

## TODO: Foundation Certification Layer (Future)

### Concept
Add a 4th layer to the FAF Quality Stack: **Foundation Certification**

### Components to Build
- [ ] Certification registry (database of certified .faf files)
- [ ] Verification endpoint (`GET /verify/{hash}`)
- [ ] Badge/stamp system for certified projects
- [ ] Public discovery listing
- [ ] Certification criteria (minimum tier, WJTTC pass, etc.)

### Proposed Flow
```
.faf scored → WJTTC passed → Submit to Foundation → Certification issued
                                    ↓
                            Hash recorded in registry
                                    ↓
                            Badge available for README
```

### Badge Format (Draft)
```markdown
![FAF Certified](https://faf.one/badge/{hash}.svg)
```

### Certification Tiers (Draft)
| Tier | Requirement |
|------|-------------|
| Bronze Certified | 85%+ score, WJTTC pass |
| Silver Certified | 95%+ score, WJTTC pass |
| Gold Certified | 99%+ score, WJTTC pass, manual review |
| Trophy Certified | 100% score, WJTTC pass, Foundation endorsed |

### Priority
LOW - Future milestone after core tooling stable

---

## TODO: .taf and Git Integration (Future)

### Concept
Integrate testing context (.taf) and Git operations into WJTTC pipeline

### Components to Investigate
- [ ] .taf file format for test context (like .faf for project context)
- [ ] Git hook integration (pre-commit WJTTC runs)
- [ ] Git-based test history tracking
- [ ] GitHub Actions workflow template
- [ ] GitLab CI / other CI templates
- [ ] Test result badges from CI

### Notes
- xAI doesn't use GitHub - keep Git integration optional
- .taf should work standalone without Git dependency
- Focus on universal approach first, GitHub-specific second

### Proposed .taf Structure (Draft)
```yaml
testing:
  framework: wjttc
  version: 1.0.0
  partner: cargo test

last_run:
  date: 2025-11-26
  combined_total: 64
  status: CHAMPIONSHIP GRADE

history:
  - date: 2025-11-26
    passed: 64
    failed: 0
```

### Priority
LOW - After xAI deal lands, GitHub integration for broader adoption

---

*WJTTC - Championship Grade Testing*
*"When brakes must work flawlessly, so must our code"*
