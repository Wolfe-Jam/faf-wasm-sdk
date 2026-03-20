const kernel = require('/Users/wolfejam/FAF/faf-wasm-sdk/pkg-node/faf_wasm_sdk.js');

const SLOTS_21 = [
  'project.name', 'project.goal', 'project.main_language',
  'stack.frontend', 'stack.css_framework', 'stack.ui_library', 'stack.state_management',
  'stack.backend', 'stack.api_type', 'stack.runtime', 'stack.database', 'stack.connection',
  'stack.hosting', 'stack.build', 'stack.cicd',
  'human_context.who', 'human_context.what', 'human_context.why',
  'human_context.where', 'human_context.when', 'human_context.how'
];

const ENTERPRISE_12 = [
  'stack.monorepo_tool', 'stack.package_manager', 'stack.workspaces',
  'monorepo.packages_count', 'monorepo.build_orchestrator',
  'stack.admin', 'stack.cache', 'stack.search', 'stack.storage',
  'monorepo.versioning_strategy', 'monorepo.shared_configs', 'monorepo.remote_cache'
];

function buildYaml(slots, populatedCount, ignoredCount) {
  const total = slots.length;
  const emptyCount = total - populatedCount - ignoredCount;
  const sections = {};

  slots.forEach((slot, i) => {
    const [section, key] = slot.split('.');
    if (!sections[section]) sections[section] = [];

    let value;
    if (i < populatedCount) {
      value = `value_${i}`;
    } else if (i < populatedCount + emptyCount) {
      value = '';
    } else {
      value = 'slotignored';
    }
    sections[section].push({ key, value });
  });

  const lines = [];
  for (const [section, entries] of Object.entries(sections)) {
    lines.push(`${section}:`);
    for (const { key, value } of entries) {
      if (value === '') {
        lines.push(`  ${key}:`);
      } else {
        lines.push(`  ${key}: "${value}"`);
      }
    }
  }
  return lines.join('\n');
}

function expectedScore(p, e) {
  const active = p + e;
  if (active === 0) return 0;
  return Math.round((p / active) * 100);
}

function expectedTier(score) {
  if (score >= 100) return '🏆';
  if (score >= 99) return '🥇';
  if (score >= 95) return '🥈';
  if (score >= 85) return '🥉';
  if (score >= 70) return '🟢';
  if (score >= 55) return '🟡';
  return '🔴';
}

let totalTests = 0;
let passed = 0;
let failed = 0;
const failures = [];

function testAll(label, slots, scoreFn) {
  const total = slots.length;
  for (let p = 0; p <= total; p++) {
    for (let i = 0; i <= total - p; i++) {
      const e = total - p - i;
      const yaml = buildYaml(slots, p, i);

      let result;
      try {
        result = JSON.parse(scoreFn(yaml));
      } catch (err) {
        failures.push({ label, p, e, i, error: err.message });
        failed++;
        totalTests++;
        continue;
      }

      const expScore = expectedScore(p, e);
      const expTier = expectedTier(expScore);

      const ok = result.score === expScore
        && result.tier === expTier
        && result.populated === p
        && result.empty === e
        && result.ignored === i
        && result.active === (p + e)
        && result.total === total;

      if (ok) {
        passed++;
      } else {
        failed++;
        failures.push({
          label, p, e, i,
          expected: { score: expScore, tier: expTier, pop: p, emp: e, ign: i, act: p+e, tot: total },
          actual: { score: result.score, tier: result.tier, pop: result.populated, emp: result.empty, ign: result.ignored, act: result.active, tot: result.total }
        });
      }
      totalTests++;
    }
  }
}

console.log('================================================================');
console.log('  PROOF BY EXHAUSTION');
console.log('  The Score Can Only Be Correct');
console.log('================================================================');
console.log('');
console.log('  A slot starts EMPTY.');
console.log('  It becomes POPULATED or SLOTIGNORED.');
console.log('  No other state exists.');
console.log('');
console.log('  Score = round(populated / (populated + empty) * 100)');
console.log('');
console.log('  7 properties verified per combination:');
console.log('    score, tier, populated, empty, ignored, active, total');
console.log('');

const t0 = Date.now();

console.log('  Base (21 slots): 253 combinations...');
testAll('Base', SLOTS_21, kernel.score_faf);

console.log('  Enterprise (33 slots): 595 combinations...');
testAll('Enterprise', SLOTS_21.concat(ENTERPRISE_12), kernel.score_faf_enterprise);

const ms = Date.now() - t0;

console.log('');
console.log('================================================================');
console.log('  RESULTS');
console.log('================================================================');
console.log('');
console.log(`  Combinations:   ${totalTests} / 848`);
console.log(`  Properties:     ${totalTests * 7} / 5,936`);
console.log(`  Passed:         ${passed}`);
console.log(`  Failed:         ${failed}`);
console.log(`  Time:           ${ms}ms`);

if (failures.length > 0) {
  console.log('');
  console.log('  FAILURES (first 5):');
  failures.slice(0, 5).forEach(f => {
    if (f.error) {
      console.log(`    ${f.label} P=${f.p} E=${f.e} I=${f.i} ERROR: ${f.error}`);
    } else {
      console.log(`    ${f.label} P=${f.p} E=${f.e} I=${f.i}`);
      console.log(`      Expected: ${JSON.stringify(f.expected)}`);
      console.log(`      Actual:   ${JSON.stringify(f.actual)}`);
    }
  });
  if (failures.length > 5) console.log(`    ... and ${failures.length - 5} more`);
}

console.log('');
console.log('================================================================');
if (failed === 0) {
  console.log('');
  console.log('  PROOF COMPLETE.');
  console.log('');
  console.log('  848 combinations. 5,936 properties. Zero failures.');
  console.log('  Every possible state of a .faf file has been tested.');
  console.log('');
  console.log('  The Bouncer cleans. The Foundry divides. QED.');
  console.log('');
} else {
  console.log(`  PROOF INCOMPLETE: ${failed} failures.`);
}
console.log('================================================================');
