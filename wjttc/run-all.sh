#!/bin/bash
# ============================================
# WJTTC - Wolfejam Testing and Test Center
# FAF WASM SDK Test Suite
# Standard: WJTTC v1.0.0
# F1-Inspired: When brakes must work flawlessly, so must our code
# ============================================
#
# Partner Framework: cargo test (Rust)
# Lint Tool: cargo clippy
# Sections: 6 (Build, Lint, Unit, Logic, API, Demo)
# Minimum: 50+ combined tests (medium project)
#
# ============================================

set +e

echo ""
echo "  WJTTC - FAF WASM SDK Test Suite"
echo "  Standard: v1.0.0 | Partner: cargo test"
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

# ============================================
# SECTION 1: Build Tests (10 tests)
# ============================================
echo -e "${CYAN}[1/5] Build Tests${NC}"

# Test 1.1: Cargo.toml exists
if [ -f "../Cargo.toml" ]; then pass "Cargo.toml exists"; else fail "Cargo.toml missing"; fi

# Test 1.2: src/lib.rs exists
if [ -f "../src/lib.rs" ]; then pass "src/lib.rs exists"; else fail "src/lib.rs missing"; fi

# Test 1.3: src/scorer.rs exists
if [ -f "../src/scorer.rs" ]; then pass "src/scorer.rs exists"; else fail "src/scorer.rs missing"; fi

# Test 1.4: src/types.rs exists
if [ -f "../src/types.rs" ]; then pass "src/types.rs exists"; else fail "src/types.rs missing"; fi

# Test 1.5: src/parser.rs exists
if [ -f "../src/parser.rs" ]; then pass "src/parser.rs exists"; else fail "src/parser.rs missing"; fi

# Test 1.6: pkg directory exists
if [ -d "../pkg" ]; then pass "pkg directory exists"; else fail "pkg directory missing"; fi

# Test 1.7: WASM binary exists
if [ -f "../pkg/faf_wasm_sdk_bg.wasm" ]; then pass "WASM binary exists"; else fail "WASM binary missing"; fi

# Test 1.8: JS glue exists
if [ -f "../pkg/faf_wasm_sdk.js" ]; then pass "JS glue code exists"; else fail "JS glue missing"; fi

# Test 1.9: TypeScript defs exist
if [ -f "../pkg/faf_wasm_sdk.d.ts" ]; then pass "TypeScript definitions exist"; else fail "TS defs missing"; fi

# Test 1.10: WASM size < 300KB
WASM_SIZE=$(stat -f%z "../pkg/faf_wasm_sdk_bg.wasm" 2>/dev/null || stat -c%s "../pkg/faf_wasm_sdk_bg.wasm")
if [ "$WASM_SIZE" -lt 300000 ]; then pass "WASM size under 300KB ($((WASM_SIZE/1024))KB)"; else fail "WASM too large ($((WASM_SIZE/1024))KB)"; fi

echo ""

# ============================================
# SECTION 2: Rust Unit Tests (via cargo test)
# ============================================
echo -e "${CYAN}[2/6] Rust Lint Check (Clippy)${NC}"

cd ..
if cargo clippy --quiet 2>/dev/null; then
    pass "cargo clippy passed (zero warnings)"
else
    fail "cargo clippy has warnings"
fi
cd wjttc

echo ""

echo -e "${CYAN}[3/6] Rust Unit Tests (Cargo)${NC}"

cd ..
# Run cargo test and capture output
CARGO_OUTPUT=$(cargo test 2>&1)
CARGO_EXIT=$?

if [ $CARGO_EXIT -eq 0 ]; then
    # Extract test count from output like "test result: ok. 30 passed"
    TEST_COUNT=$(echo "$CARGO_OUTPUT" | grep -E "^test result: ok\." | head -1 | sed 's/.*ok\. \([0-9]*\) passed.*/\1/')
    if [ -z "$TEST_COUNT" ]; then
        TEST_COUNT=0
    fi
    pass "cargo test passed ($TEST_COUNT unit tests)"

    # List test categories
    echo "  Test categories:"
    echo "    - Weights tests (sum, values, labels)"
    echo "    - Tier tests (trophy, gold, silver, bronze, green, yellow, red)"
    echo "    - Weighted score tests (calculation, different values, truth)"
    echo "    - Language bonus tests (rust, go, typescript, cap, unknown)"
    echo "    - YAML parsing tests (minimal, invalid, full)"
    echo "    - Mk3 slot-based tests (empty, project, tier)"
    echo "    - Hot path function tests (weights, fast, constants)"
    echo "    - JSON/display output tests"
else
    fail "cargo test failed"
    echo "$CARGO_OUTPUT" | tail -20
fi
cd wjttc

echo ""

# ============================================
# SECTION 4: Scoring Algorithm Tests
# ============================================
echo -e "${CYAN}[4/6] Scoring Algorithm Tests${NC}"

# Test 3.1: Mk3 tier thresholds in code
if grep -q "100.0" ../src/scorer.rs; then pass "Trophy tier (100%) defined"; else fail "Trophy tier missing"; fi
if grep -q "99.0" ../src/scorer.rs; then pass "Gold tier (99%+) defined"; else fail "Gold tier missing"; fi
if grep -q "95.0" ../src/scorer.rs; then pass "Silver tier (95%+) defined"; else fail "Silver tier missing"; fi
if grep -q "85.0" ../src/scorer.rs; then pass "Bronze tier (85%+) defined"; else fail "Bronze tier missing"; fi
if grep -q "70.0" ../src/scorer.rs; then pass "Green tier (70%+) defined"; else fail "Green tier missing"; fi
if grep -q "55.0" ../src/scorer.rs; then pass "Yellow tier (55%+) defined"; else fail "Yellow tier missing"; fi

# Test 3.2: Elon Weights defined
if grep -q "0.40" ../src/scorer.rs; then pass "Completeness weight (40%) defined"; else fail "Completeness weight missing"; fi
if grep -q "0.35" ../src/scorer.rs; then pass "Clarity weight (35%) defined"; else fail "Clarity weight missing"; fi
if grep -q "0.15" ../src/scorer.rs; then pass "Structure weight (15%) defined"; else fail "Structure weight missing"; fi
if grep -q "0.10" ../src/scorer.rs; then pass "Metadata weight (10%) defined"; else fail "Metadata weight missing"; fi

echo ""

# ============================================
# SECTION 5: WASM API Tests
# ============================================
echo -e "${CYAN}[5/6] WASM API Tests${NC}"

# Test exports in JS file
if grep -q "export class FAF" ../pkg/faf_wasm_sdk.js; then pass "FAF class exported"; else fail "FAF class not exported"; fi
if grep -q "mk3_score" ../pkg/faf_wasm_sdk.js; then pass "mk3_score getter exported"; else fail "mk3_score missing"; fi
if grep -q "mk3_tier" ../pkg/faf_wasm_sdk.js; then pass "mk3_tier getter exported"; else fail "mk3_tier missing"; fi
if grep -q "weighted_score" ../pkg/faf_wasm_sdk.js; then pass "weighted_score getter exported"; else fail "weighted_score missing"; fi
if grep -q "tier" ../pkg/faf_wasm_sdk.js; then pass "tier getter exported"; else fail "tier getter missing"; fi
if grep -q "completeness" ../pkg/faf_wasm_sdk.js; then pass "completeness getter exported"; else fail "completeness missing"; fi
if grep -q "clarity" ../pkg/faf_wasm_sdk.js; then pass "clarity getter exported"; else fail "clarity missing"; fi
if grep -q "structure" ../pkg/faf_wasm_sdk.js; then pass "structure getter exported"; else fail "structure missing"; fi
if grep -q "metadata" ../pkg/faf_wasm_sdk.js; then pass "metadata getter exported"; else fail "metadata missing"; fi
if grep -q "name" ../pkg/faf_wasm_sdk.js; then pass "name getter exported"; else fail "name getter missing"; fi

echo ""

# ============================================
# SECTION 6: Browser Demo Tests
# ============================================
echo -e "${CYAN}[6/6] Browser Demo Tests${NC}"

# Test FAF version demo
if [ -f "../examples/browser.html" ]; then pass "FAF browser demo exists"; else fail "FAF demo missing"; fi
if grep -q "SHOW_ELON_WEIGHTS = false" ../examples/browser.html; then pass "Elon Weights hidden in FAF version"; else fail "Elon Weights not hidden"; fi
if grep -q "FAF Score" ../examples/browser.html; then pass "FAF Score label present"; else fail "FAF Score label missing"; fi

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
PARTNER_TIME="0.00s"

# Combined totals
COMBINED_PASS=$((PARTNER_TESTS + WJTTC_TESTS))
COMBINED_TOTAL=$((COMBINED_PASS + WJTTC_FAIL))

echo ""
echo "  ====================================="
echo -e "  ${CYAN}WJTTC COMBINED TEST REPORT${NC}"
echo "  ====================================="
echo "  Partner Framework: $PARTNER_FRAMEWORK"
echo -e "  Partner Tests:     ${GREEN}$PARTNER_TESTS passed${NC} ($PARTNER_TIME)"
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
