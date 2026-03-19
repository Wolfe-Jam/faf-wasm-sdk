# FAF WASM SDK v2

Compile, score, and inspect [FAF](https://faf.one) context — in any browser, edge runtime, or Node.js process. One WASM binary, 8 exports, ~322KB.

## v2 — The Definitive Edition

v1 proved the concept: compile YAML to binary, score it, ship it to the edge. But v1 had 11 hardcoded section types. That works for a solo project. It breaks the moment an enterprise with 680 engineers, 62 teams, and custom compliance chunks tries to describe their repo.

v2 is the format we always wanted to ship. The FAF creator fell in love with IFF in the 90s — working with the Interchange File Format that Commodore created for the Amiga across early computer graphics engines and apps. That chunked binary architecture influenced everything that came after. Microsoft literally riffed on it with RIFF. IFF got it right the first time.

FAFb v2 brings that same architecture into the AI era: a **string table** replacing the fixed enum, the same pattern ELF and IFF have used for decades. FAF creator realized every YAML key can just become a named section. No limits. No "Unknown" fallback. No artificial ceiling.

This is a significant upgrade and it's free — the SDK is MIT, the format is an IANA-registered open standard, and the binary spec is public. We're making the standard bulletproof so everyone can build on it.

### What v2 adds

- **Unlimited section names** — any YAML key becomes a binary section (up to 256 per file)
- **Chunk classification** — every section is automatically tagged as DNA (core identity), Context (supplementary), or Pointer (documentation references)
- **Enterprise scoring** — 33-slot Mk4 engine with monorepo, compliance, and team structure support
- **Deterministic output** — same YAML in, same binary out, every time (CRC32 sealed)

### What stays the same

- 32-byte header, 16-byte section entries — same wire layout
- Same 8 JavaScript exports, same function signatures
- Same sub-2ms parse + score performance
- Priority truncation for context window management
- IANA-registered `application/vnd.faf+yaml` input format

## Installation

```bash
npm install @faf/wasm-sdk
```

## API

8 pure functions. No classes. JSON in, JSON out.

```javascript
import init, {
  sdk_version,
  score_faf,
  score_faf_enterprise,
  validate_faf,
  compile_fafb,
  decompile_fafb,
  score_fafb,
  fafb_info
} from '@faf/wasm-sdk';

await init();
```

### Score YAML

```javascript
const result = JSON.parse(score_faf(yamlContent));
// { score: 71, tier: "🟢", populated: 15, total: 21, ... }

// Enterprise orgs: 33-slot scoring
const enterprise = JSON.parse(score_faf_enterprise(yamlContent));
// { score: 45, tier: "🟡", populated: 15, total: 33, ... }
```

### Compile to Binary

```javascript
const bytes = compile_fafb(yamlContent);   // Uint8Array
const json = decompile_fafb(bytes);        // JSON string with all sections + content
const info = fafb_info(bytes);             // JSON string, metadata only (no content)
const score = score_fafb(bytes);           // JSON string, score from embedded meta
```

### Validate

```javascript
validate_faf(yamlContent);  // true if valid YAML mapping
```

### Edge Runtime

```javascript
export default {
  async fetch(request) {
    await init();
    const score = score_faf(yamlContent);
    return Response.json(JSON.parse(score));
  }
};
```

## Scoring

Mk4 engine — the same scorer that runs in the Rust SDK and CLI. Slot-based: each YAML key is a slot, populated slots are counted, placeholders and empty values are rejected.

| Score | Tier |
|-------|------|
| 100% | 🏆 Championship |
| 99% | 🥇 Gold |
| 95% | 🥈 Silver |
| 85% | 🥉 Bronze |
| 70% | 🟢 Green |
| 55% | 🟡 Yellow |
| <55% | 🔴 Red |

**Base** scores against 21 slots. **Enterprise** scores against 33 (adds monorepo, compliance, team structure, and more).

## FAFb Binary Format

The compiled output. YAML is source code, FAFb is the compiled binary.

```
HEADER (32 bytes)
  Magic: "FAFB"
  Version, flags, section count, CRC32 checksum

SECTION DATA (variable)
  Each YAML key → one section, priority-ordered

STRING TABLE (appended)
  Section name index — unlimited names, O(1) lookup

SECTION TABLE (at end)
  16 bytes per entry: name index, priority, offset, length, token count, classification
```

Every section carries a **classification**:
- **DNA** — core project identity (`project`, `tech_stack`, `commands`, `faf_version`, ...)
- **Context** — supplementary chunks (`compliance`, `security`, `agents`, custom fields, ...)
- **Pointer** — documentation references (`docs`)

## Build

```bash
cargo install wasm-pack
wasm-pack build --target web --release

# Tests
cargo test
```

## Tests

138 tests across 4 suites:
- **Unit tests** (90) — scoring, compilation, decompilation, edge cases
- **Stress tests** (27) — YAML bombs, Unicode, large payloads, binary fuzzing
- **WASM integration** (21) — public API surface through wasm-bindgen

## License

MIT

## Links

- [faf.one](https://faf.one) — project home
- [IANA Registration](https://www.iana.org/assignments/media-types/application/vnd.faf+yaml) — `application/vnd.faf+yaml`
- [FAF on Zenodo](https://doi.org/10.5281/zenodo.18251362) — academic paper (DOI 10.5281/zenodo.18251362)
- [FAF on Grokipedia](https://grokipedia.com/page/faf-file-format) — 28 citations
- [faf-rust-sdk](https://github.com/faf-foundation/faf-rust-sdk) — canonical Rust implementation
