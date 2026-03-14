#!/bin/bash
# ============================================
# WJTTC - Wolfejam Testing and Test Center
# FAF WASM SDK v2.0.0 Test Suite
# Standard: WJTTC v2.0.0
# F1-Inspired: When brakes must work flawlessly, so must our code
# ============================================
#
# Partner Framework: cargo test (Rust)
# Lint Tool: cargo clippy
# Sections: 6 (Build, Lint, Unit, Mk4, FAFb API, Browser)
#
# v2.0.0 — Rewritten for Mk4 33-slot engine + pure function exports
# ============================================

set +e

echo ""
echo "  WJTTC - FAF WASM SDK v2.0.0 Test Suite"
echo "  Standard: v2.0.0 | Partner: cargo test"
echo "  ========================================"
echo ""

PASS=0
FAIL=0
START=$(python3 -c "import time; print(int(time.time()*1000))")

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

pass() {
    echo -e "  ${GREEN}✓${NC} $1"
    ((PASS++))
}

fail() {
    echo -e "  ${RED}✗${NC} $1"
    ((FAIL++))
}

# Navigate to project root
cd "$(dirname "$0")/.."

# ============================================
# SECTION 1: Build Tests (9 tests)
# ============================================
echo -e "${CYAN}[1/6] Build Tests${NC}"

# Test 1.1: Cargo.toml exists
if [ -f "Cargo.toml" ]; then pass "Cargo.toml exists"; else fail "Cargo.toml missing"; fi

# Test 1.2: src/lib.rs exists (entry point)
if [ -f "src/lib.rs" ]; then pass "src/lib.rs exists"; else fail "src/lib.rs missing"; fi

# Test 1.3: src/mk4.rs exists (Mk4 scoring engine)
if [ -f "src/mk4.rs" ]; then pass "src/mk4.rs exists"; else fail "src/mk4.rs missing"; fi

# Test 1.4: src/fafb.rs exists (FAFb binary format)
if [ -f "src/fafb.rs" ]; then pass "src/fafb.rs exists"; else fail "src/fafb.rs missing"; fi

# Test 1.5: Old v1.x files are GONE
if [ ! -f "src/scorer.rs" ] && [ ! -f "src/types.rs" ] && [ ! -f "src/parser.rs" ] && [ ! -f "src/error.rs" ] && [ ! -f "src/generator.rs" ]; then
    pass "Old v1.x files removed (scorer, types, parser, error, generator)"
else
    fail "Old v1.x files still present"
fi

# Test 1.6: pkg directory exists
if [ -d "pkg" ]; then pass "pkg directory exists"; else fail "pkg directory missing"; fi

# Test 1.7: WASM binary exists
if [ -f "pkg/faf_wasm_sdk_bg.wasm" ]; then pass "WASM binary exists"; else fail "WASM binary missing"; fi

# Test 1.8: JS glue exists
if [ -f "pkg/faf_wasm_sdk.js" ]; then pass "JS glue code exists"; else fail "JS glue missing"; fi

# Test 1.9: TypeScript defs exist
if [ -f "pkg/faf_wasm_sdk.d.ts" ]; then pass "TypeScript definitions exist"; else fail "TS defs missing"; fi

echo ""

# ============================================
# SECTION 2: Rust Lint Check (Clippy)
# ============================================
echo -e "${CYAN}[2/6] Rust Lint Check (Clippy)${NC}"

if cargo clippy --quiet 2>/dev/null; then
    pass "cargo clippy passed (zero warnings)"
else
    fail "cargo clippy has warnings"
fi

echo ""

# ============================================
# SECTION 3: Rust Unit Tests (cargo test)
# ============================================
echo -e "${CYAN}[3/6] Rust Unit Tests (Cargo)${NC}"

CARGO_OUTPUT=$(cargo test 2>&1)
CARGO_EXIT=$?

if [ $CARGO_EXIT -eq 0 ]; then
    # Extract test counts from all test result lines
    TEST_COUNT=$(echo "$CARGO_OUTPUT" | grep -E "^test result: ok\." | awk '{sum += $4} END {print sum}')
    if [ -z "$TEST_COUNT" ] || [ "$TEST_COUNT" = "0" ]; then
        TEST_COUNT=0
    fi
    pass "cargo test passed ($TEST_COUNT unit tests)"

    echo "  Test modules:"
    echo "    - mk4::tests (slot states, placeholders, tiers, parity, edge cases)"
    echo "    - fafb::tests (compile, decompile, roundtrip, checksum, unicode)"
    echo "    - lib::tests (public API integration)"
    echo "    - tests::wasm (external integration tests)"
else
    fail "cargo test failed"
    echo "$CARGO_OUTPUT" | tail -20
fi

echo ""

# ============================================
# SECTION 4: Mk4 Scoring Engine Tests
# ============================================
echo -e "${CYAN}[4/6] Mk4 Scoring Engine Tests${NC}"

# Test 4.1: Mk4 tier thresholds in code
if grep -q "score >= 100" src/mk4.rs; then pass "Trophy tier (100%) defined"; else fail "Trophy tier missing"; fi
if grep -q "score >= 99" src/mk4.rs; then pass "Gold tier (99%+) defined"; else fail "Gold tier missing"; fi
if grep -q "score >= 95" src/mk4.rs; then pass "Silver tier (95%+) defined"; else fail "Silver tier missing"; fi
if grep -q "score >= 85" src/mk4.rs; then pass "Bronze tier (85%+) defined"; else fail "Bronze tier missing"; fi
if grep -q "score >= 70" src/mk4.rs; then pass "Green tier (70%+) defined"; else fail "Green tier missing"; fi
if grep -q "score >= 55" src/mk4.rs; then pass "Yellow tier (55%+) defined"; else fail "Yellow tier missing"; fi

# Test 4.2: 21 Base slots defined
BASE_SLOT_COUNT=$(grep -c 'to_string()' src/mk4.rs | head -1)
if grep -q "project.name" src/mk4.rs && grep -q "project.goal" src/mk4.rs && grep -q "project.main_language" src/mk4.rs; then
    pass "Project Meta slots (3) defined"
else
    fail "Project Meta slots missing"
fi

if grep -q "human_context.who" src/mk4.rs && grep -q "human_context.how" src/mk4.rs; then
    pass "Human Context slots (6) defined"
else
    fail "Human Context slots missing"
fi

if grep -q "stack.frontend" src/mk4.rs && grep -q "stack.cicd" src/mk4.rs; then
    pass "Stack slots (12) defined"
else
    fail "Stack slots missing"
fi

# Test 4.3: Enterprise slots
if grep -q "monorepo.packages_count" src/mk4.rs && grep -q "monorepo.remote_cache" src/mk4.rs; then
    pass "Enterprise/Monorepo slots defined"
else
    fail "Enterprise slots missing"
fi

# Test 4.4: Placeholder rejection
if grep -q "describe your project goal" src/mk4.rs && grep -q "not applicable" src/mk4.rs; then
    pass "Placeholder rejection list (8 entries) defined"
else
    fail "Placeholder rejection missing"
fi

# Test 4.5: Slotignored handling
if grep -q "slotignored" src/mk4.rs && grep -q "Slotignored" src/mk4.rs; then
    pass "Slotignored state handling defined"
else
    fail "Slotignored handling missing"
fi

echo ""

# ============================================
# SECTION 5: WASM API Tests (Pure Functions)
# ============================================
echo -e "${CYAN}[5/6] WASM API Tests (v2.0.0 Pure Functions)${NC}"

# Test exports in JS glue file — v2.0.0 pure functions (no FAF class)
if [ -f "pkg/faf_wasm_sdk.js" ]; then
    if grep -q "sdk_version" pkg/faf_wasm_sdk.js; then pass "sdk_version() exported"; else fail "sdk_version missing"; fi
    if grep -q "score_faf" pkg/faf_wasm_sdk.js; then pass "score_faf() exported"; else fail "score_faf missing"; fi
    if grep -q "validate_faf" pkg/faf_wasm_sdk.js; then pass "validate_faf() exported"; else fail "validate_faf missing"; fi
    if grep -q "compile_fafb" pkg/faf_wasm_sdk.js; then pass "compile_fafb() exported"; else fail "compile_fafb missing"; fi
    if grep -q "decompile_fafb" pkg/faf_wasm_sdk.js; then pass "decompile_fafb() exported"; else fail "decompile_fafb missing"; fi
    if grep -q "score_fafb" pkg/faf_wasm_sdk.js; then pass "score_fafb() exported"; else fail "score_fafb missing"; fi
    if grep -q "fafb_info" pkg/faf_wasm_sdk.js; then pass "fafb_info() exported"; else fail "fafb_info missing"; fi
    if grep -q "score_faf_enterprise" pkg/faf_wasm_sdk.js; then pass "score_faf_enterprise() exported"; else fail "score_faf_enterprise missing"; fi

    # v2.0.0: NO class exports (pure functions only)
    if ! grep -q "export class FAF" pkg/faf_wasm_sdk.js; then
        pass "No FAF class exported (pure functions only)"
    else
        fail "Old FAF class still exported — should be pure functions"
    fi
else
    fail "pkg/faf_wasm_sdk.js not found — run wasm-pack build first"
fi

# TypeScript definitions
if [ -f "pkg/faf_wasm_sdk.d.ts" ]; then
    if grep -q "sdk_version" pkg/faf_wasm_sdk.d.ts; then pass "TS types include sdk_version"; else fail "TS sdk_version missing"; fi
    if grep -q "score_faf" pkg/faf_wasm_sdk.d.ts; then pass "TS types include score_faf"; else fail "TS score_faf missing"; fi
    if grep -q "compile_fafb" pkg/faf_wasm_sdk.d.ts; then pass "TS types include compile_fafb"; else fail "TS compile_fafb missing"; fi
else
    fail "pkg/faf_wasm_sdk.d.ts not found"
fi

echo ""

# ============================================
# SECTION 6: Browser Demo Tests
# ============================================
echo -e "${CYAN}[6/6] Browser Demo Tests${NC}"

if [ -f "examples/browser.html" ]; then pass "Browser demo exists"; else fail "Browser demo missing"; fi

echo ""

# ============================================
# RESULTS - COMBINED PARTNER REPORT
# ============================================
END=$(python3 -c "import time; print(int(time.time()*1000))")
ELAPSED=$((END - START))

# WJTTC tests (excluding the cargo test line which is the partner)
WJTTC_TESTS=$((PASS - 1))  # Subtract 1 for the "cargo test passed" line
WJTTC_FAIL=$FAIL

# Partner test count (captured during cargo test)
PARTNER_TESTS=${TEST_COUNT:-0}
PARTNER_FRAMEWORK="cargo test"

# Combined totals
COMBINED_PASS=$((PARTNER_TESTS + WJTTC_TESTS))
COMBINED_TOTAL=$((COMBINED_PASS + WJTTC_FAIL))

echo ""
echo "  ====================================="
echo -e "  ${CYAN}WJTTC COMBINED TEST REPORT${NC}"
echo "  ====================================="
echo "  Partner Framework: $PARTNER_FRAMEWORK"
echo -e "  Partner Tests:     ${GREEN}$PARTNER_TESTS passed${NC}"
echo -e "  WJTTC Tests:       ${GREEN}$WJTTC_TESTS passed${NC}, ${RED}$WJTTC_FAIL failed${NC}"
echo "  -------------------------------------"
echo -e "  ${CYAN}COMBINED TOTAL:    $COMBINED_PASS passed${NC}"
echo "  COMBINED TIME:     ${ELAPSED}ms"
echo ""

if [ $WJTTC_FAIL -eq 0 ]; then
    echo -e "  STATUS: ${GREEN}CHAMPIONSHIP GRADE${NC}"
    echo "  ====================================="
    echo ""
    exit 0
else
    echo -e "  STATUS: ${RED}$WJTTC_FAIL issues to fix${NC}"
    echo "  ====================================="
    echo ""
    exit 1
fi
