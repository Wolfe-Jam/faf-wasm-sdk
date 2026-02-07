# faf-wasm

**FAF WASM SDK** - Score .faf files in Node.js (<5ms)

IANA-registered format: `application/vnd.faf+yaml`

## Installation

```bash
npm install faf-wasm
```

## Usage

### Node.js (CommonJS)

```javascript
const { FAF, validate_faf, sdk_version } = require('faf-wasm');

const faf = new FAF(`
project:
  name: my-project
  description: Example project
  stack: typescript
`);

console.log(faf.mk3_score);  // 33.3
console.log(faf.mk3_tier);   // "🔴"
console.log(faf.name);       // "my-project"
console.log(faf.mk3_display()); // "🔴 33% (3/9 slots)"
```

### Node.js (ESM)

```javascript
import { FAF } from 'faf-wasm';

const faf = new FAF(yamlContent);
console.log(faf.mk3_score, faf.mk3_tier);
```

## API

### `new FAF(yamlContent: string)`

Parse and score a .faf file.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `mk3_score` | `number` | Score percentage (0-100) |
| `mk3_tier` | `string` | Tier name (Trophy, Gold, Silver, Bronze, Green, Yellow, Red) |
| `mk3_filled` | `number` | Slots filled |
| `mk3_total` | `number` | Total slots (21) |
| `name` | `string` | Project name |
| `stack` | `string` | Project stack |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `mk3_display()` | `string` | Formatted score display |
| `mk3_breakdown()` | `string` | Detailed breakdown |
| `score_json()` | `string` | JSON export |
| `display()` | `string` | Display string |
| `FAF.validate(yaml)` | `boolean` | Validate .faf content |
| `FAF.version()` | `string` | SDK version |

## Tier System

| Tier | Score | Emoji |
|------|-------|-------|
| Trophy | 100% | 🏆 |
| Gold | 99%+ | 🥇 |
| Silver | 95%+ | 🥈 |
| Bronze | 85%+ | 🥉 |
| Green | 70%+ | 🟢 |
| Yellow | 55%+ | 🟡 |
| Red | <55% | 🔴 |
| White | 0% | 🤍 |

## Performance

- Parse + Score: <5ms
- WASM Size: 207KB
- Zero dependencies

## Links

- [FAF Format Spec](https://faf.one)
- [MCP Server](https://www.npmjs.com/package/claude-faf-mcp)

## License

FAF Foundation License v1.0 - See LICENSE file.

Free for personal, hobby, development, and internal use.
Commercial license required for offering as a service.

Contact: team@faf.one
