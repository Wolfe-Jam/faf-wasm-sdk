# FAF WASM SDK

AI Context Format for Browsers and Edge Compute. Sub-10ms context loads globally.

## Features

- YAML parsing via serde_yaml (WASM-proven)
- Glass Hood AI-readiness scoring
- 8-tier rating system (🤍 to 🏆)
- Cloudflare Workers / Vercel Edge ready
- <2ms parse + score performance

## Installation

```bash
npm install @faf/wasm-sdk
```

## Usage

### Browser / Node.js

```javascript
import { initialize, createFAF, scoreFAF } from '@faf/wasm-sdk';

await initialize();

const faf = createFAF(`
project:
  name: my-project
  stack: typescript
`);

console.log(faf.weighted_score);  // 45.0
console.log(faf.tier);            // 🟢
```

### Cloudflare Workers

```javascript
import init, { FAF } from '@faf/wasm-sdk';

export default {
  async fetch(request) {
    await init();
    const faf = new FAF(yamlContent);
    return Response.json({ score: faf.weighted_score });
  }
};
```

## API

### FAF Class

- `new FAF(yamlContent)` - Parse and score FAF content
- `name` - Project name
- `stack` - Technology stack
- `weighted_score` - Weighted AI-readiness (0-100)
- `truth_score` - Unweighted average (0-100)
- `tier` - Tier emoji (🤍🔴🟡🟢🥉🥈🥇🏆)
- `completeness` - Core DNA score (40% weight)
- `clarity` - Instructions score (35% weight)
- `structure` - Context score (15% weight)
- `metadata` - Metadata score (10% weight)

### Hot Path Functions

For maximum edge performance:

```javascript
import { score_weights_fast, WEIGHTS_F32 } from '@faf/wasm-sdk';

const values = new Float32Array([80, 70, 60, 50]);
const score = score_weights_fast(WEIGHTS_F32, values);
```

## Build

```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM
wasm-pack build --target web --release

# Run tests
wasm-pack test --headless --chrome
```

## Deploy to Cloudflare

```bash
npm install -g wrangler
wrangler deploy
```

## Scoring Algorithm

Glass Hood - Full transparency scoring:

| Category | Weight | Description |
|----------|--------|-------------|
| Completeness | 40% | Core project DNA fields |
| Clarity | 35% | AI instructions quality |
| Structure | 15% | Context organization |
| Metadata | 10% | Versioning and tags |

### Tier System

| Score | Tier | Emoji |
|-------|------|-------|
| 0-12 | White | 🤍 |
| 13-25 | Red | 🔴 |
| 26-38 | Yellow | 🟡 |
| 39-51 | Green | 🟢 |
| 52-64 | Bronze | 🥉 |
| 65-77 | Silver | 🥈 |
| 78-90 | Gold | 🥇 |
| 91-100 | Championship | 🏆 |

## License

MIT

## Links

- [FAF Format](https://faf.one)
- [IANA Registration](https://www.iana.org/assignments/media-types/application/vnd.faf+yaml)
