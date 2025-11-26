// FAF WASM SDK - JavaScript wrapper
// @faf/wasm-sdk

import init, {
  FAF,
  parse_faf,
  validate_faf,
  score_faf,
  sdk_version,
  // xAI/Grok hot path functions
  score_weights,
  score_weights_fast,
} from '../pkg/faf_wasm_sdk.js';

let initialized = false;

/**
 * Initialize WASM module (call once before use)
 */
export async function initialize() {
  if (!initialized) {
    await init();
    initialized = true;
  }
}

/**
 * Create FAF instance from YAML content
 * @param {string} yamlContent - .faf file content
 * @returns {FAF} FAF instance with scoring
 */
export function createFAF(yamlContent) {
  if (!initialized) {
    throw new Error('Call initialize() before using FAF');
  }
  return new FAF(yamlContent);
}

/**
 * Validate FAF YAML (quick check)
 * @param {string} yamlContent
 * @returns {boolean}
 */
export function isValidFAF(yamlContent) {
  if (!initialized) {
    throw new Error('Call initialize() before using FAF');
  }
  return validate_faf(yamlContent);
}

/**
 * Score FAF content (returns JSON)
 * @param {string} yamlContent
 * @returns {object} Score object
 */
export function scoreFAF(yamlContent) {
  if (!initialized) {
    throw new Error('Call initialize() before using FAF');
  }
  const json = score_faf(yamlContent);
  return JSON.parse(json);
}

/**
 * Get SDK version
 * @returns {string}
 */
export function version() {
  return sdk_version();
}

// Re-export WASM types
export { FAF };
export { init };

// =============================================================================
// HOT PATH EXPORTS - xAI/Grok Recommended
// Direct access to pre-compiled weight scoring for edge compute
// =============================================================================

/**
 * Fast weight scoring - xAI/Grok hot path
 * Uses f32 for maximum WASM performance
 * @param {Float32Array} weights - 4 weights [0.40, 0.35, 0.15, 0.10]
 * @param {number} base - Base score value
 * @returns {number} Weighted score (capped at 100)
 */
export { score_weights };

/**
 * Fast weighted calculation with separate values
 * Hot path for edge compute
 * @param {Float32Array} weights - 4 weights
 * @param {Float32Array} values - 4 category scores [completeness, clarity, structure, metadata]
 * @returns {number} Weighted score (capped at 100)
 */
export { score_weights_fast };

/**
 * Default weights as Float32Array for hot path usage
 * [0.40, 0.35, 0.15, 0.10]
 */
export const WEIGHTS_F32 = new Float32Array([0.40, 0.35, 0.15, 0.10]);
