# TODO: Port Smart Name Extraction to faf-cli

## Context
We added smart repo name extraction to the Rust WASM generator (`generator.rs`).
This logic should also be ported to faf-cli (TypeScript) for consistency.

## What to Port

**From:** `/Users/wolfejam/FAF/faf-wasm-sdk/src/generator.rs`

**Functions:**
1. `is_descriptive_name()` - Checks if repo name is descriptive
2. `infer_why_from_name()` - Infers WHY from descriptive names

## Integration Points in faf-cli

### 1. `faf readme` Enhancement
**File:** `/Users/wolfejam/FAF/cli/src/commands/readme.ts` (or similar)

When enhancing README with `faf readme --apply`, use smart name extraction to:
- Suggest WHY section content
- Pre-fill purpose/mission if missing
- Provide intelligent defaults based on repo name

### 2. `faf init` / `faf go`
**Files:** `/Users/wolfejam/FAF/cli/src/commands/init.ts`, `go.ts`

When initializing or improving project.faf:
- Use same logic to infer WHY from repo name
- Consistent behavior with builder.faf.one
- Same conservative approach (only descriptive names)

## Implementation Notes

**Logic to Port (TypeScript):**

```typescript
function isDescriptiveName(name: string): boolean {
  // Must have separators (compound structure)
  if (!name.includes('-') && !name.includes('_')) {
    return false;
  }

  const nameLower = name.toLowerCase();

  // Keywords that indicate descriptive naming
  const techKeywords = [
    'react', 'vue', 'svelte', 'angular', 'next',
    'node', 'express', 'django', 'flask',
    'api', 'cli', 'app', 'lib', 'sdk', 'tool', 'framework',
    'rust', 'python', 'go', 'java', 'js', 'ts',
  ];

  const actionVerbs = [
    'test', 'demo', 'build', 'parse', 'fetch', 'sync',
    'deploy', 'manage', 'create', 'generate', 'validate',
  ];

  const domainNouns = [
    'todo', 'auth', 'blog', 'chat', 'dashboard', 'admin',
    'user', 'data', 'file', 'image', 'video', 'audio',
  ];

  const purposeWords = [
    'demo', 'example', 'starter', 'template', 'boilerplate',
    'sample', 'tutorial', 'guide', 'playground',
  ];

  // Check if name contains known keywords
  const hasKeywords =
    techKeywords.some(k => nameLower.includes(k)) ||
    actionVerbs.some(k => nameLower.includes(k)) ||
    domainNouns.some(k => nameLower.includes(k)) ||
    purposeWords.some(k => nameLower.includes(k));

  return hasKeywords;
}

function inferWhyFromName(name: string): string | null {
  if (!isDescriptiveName(name)) {
    return null; // Name is abstract, don't infer
  }

  const nameLower = name.toLowerCase();
  const parts = nameLower.split(/[-_]/);

  // Pattern matching for common structures
  if (parts.length < 2) {
    return null;
  }

  // Check for common patterns
  if (parts.includes('demo') || parts.includes('example')) {
    const filtered = parts.filter(p =>
      p !== 'demo' && p !== 'example' && p !== 'test'
    );
    return `Demonstrate and test ${filtered.join(' ')} system`;
  }

  if (parts.includes('starter') || parts.includes('template') || parts.includes('boilerplate')) {
    const filtered = parts.filter(p =>
      p !== 'starter' && p !== 'template' && p !== 'boilerplate'
    );
    return `Starter template for ${filtered.join(' ')} development`;
  }

  // Generic descriptive inference
  const readable = parts.join(' ');
  return `Project for ${readable}`;
}
```

## Testing

**Test Cases:**
- `test-faf-demo` → "Demonstrate and test faf system" ✅
- `react-todo-app` → Should infer purpose ✅
- `ziggy` → null (abstract) ✅
- `kubernetes` → null (abstract) ✅
- `my-awesome-project` → null (generic) ✅

## Priority
**Medium** - Not urgent, but valuable for consistency across the ecosystem.

## Benefits
1. **Consistency:** Same smart extraction in WASM and CLI
2. **Better UX:** faf-cli can suggest better defaults
3. **Less manual work:** Intelligent README enhancement
4. **Unified logic:** Single source of truth for extraction patterns

---

**Status:** TODO (not yet implemented in faf-cli)
**Implemented in:** faf-wasm-sdk v1.0.1 (2026-02-05)
