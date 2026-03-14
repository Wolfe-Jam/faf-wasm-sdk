//! FAF WASM SDK v2.0.0 — AI Context Format for Edge Compute
//!
//! 7 pure-function exports. No classes. JSON in, JSON out.
//!
//! # Usage (JavaScript)
//! ```js
//! import init, { sdk_version, score_faf, validate_faf,
//!     compile_fafb, decompile_fafb, score_fafb, fafb_info } from '@faf/wasm-sdk';
//!
//! await init();
//!
//! // Score YAML
//! const result = score_faf(yamlContent);  // JSON string
//!
//! // Compile to binary
//! const bytes = compile_fafb(yamlContent);  // Uint8Array
//!
//! // Decompile binary
//! const json = decompile_fafb(bytes);  // JSON string
//! ```

pub mod mk4;
pub mod fafb;

use wasm_bindgen::prelude::*;
use mk4::{Mk4Scorer, LicenseTier};

// =============================================================================
// THE 7 EXPORTS — Pure functions, no classes
// =============================================================================

/// Get SDK version
#[wasm_bindgen]
pub fn sdk_version() -> String {
    "2.0.0".to_string()
}

/// Score FAF YAML content using Mk4 engine — returns JSON
#[wasm_bindgen]
pub fn score_faf(yaml: String) -> Result<String, JsValue> {
    let scorer = Mk4Scorer::new(LicenseTier::Base);
    let result = scorer
        .calculate(&yaml)
        .map_err(|e| JsValue::from_str(&e))?;
    Ok(result.to_json())
}

/// Validate FAF YAML content — returns true if parseable as YAML mapping
#[wasm_bindgen]
pub fn validate_faf(yaml: String) -> bool {
    use serde_yaml::Value;
    matches!(serde_yaml::from_str::<Value>(&yaml), Ok(Value::Mapping(_)))
}

/// Compile YAML to FAFb binary — returns Uint8Array
#[wasm_bindgen]
pub fn compile_fafb(yaml: String) -> Result<Vec<u8>, JsValue> {
    fafb::compile_fafb(&yaml).map_err(|e| JsValue::from_str(&e))
}

/// Decompile FAFb binary to JSON (full content) — returns JSON string
#[wasm_bindgen]
pub fn decompile_fafb(bytes: &[u8]) -> Result<String, JsValue> {
    fafb::decompile_fafb(bytes).map_err(|e| JsValue::from_str(&e))
}

/// Score a FAFb binary file — returns JSON string
#[wasm_bindgen]
pub fn score_fafb(bytes: &[u8]) -> Result<String, JsValue> {
    fafb::score_fafb(bytes).map_err(|e| JsValue::from_str(&e))
}

/// Get FAFb file info (header + section metadata, no content) — returns JSON string
#[wasm_bindgen]
pub fn fafb_info(bytes: &[u8]) -> Result<String, JsValue> {
    fafb::fafb_info(bytes).map_err(|e| JsValue::from_str(&e))
}

// =============================================================================
// ENTERPRISE SCORING — same 7 pattern, different tier
// =============================================================================

/// Score FAF YAML with enterprise (33-slot) tier — returns JSON
#[wasm_bindgen]
pub fn score_faf_enterprise(yaml: String) -> Result<String, JsValue> {
    let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
    let result = scorer
        .calculate(&yaml)
        .map_err(|e| JsValue::from_str(&e))?;
    Ok(result.to_json())
}

// =============================================================================
// INTEGRATION TESTS — Public API surface
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdk_version_value() {
        assert_eq!(sdk_version(), "2.0.0");
    }

    #[test]
    fn test_validate_faf_accepts_mapping() {
        assert!(validate_faf("project:\n  name: test".to_string()));
    }

    #[test]
    fn test_validate_faf_rejects_non_mapping() {
        assert!(!validate_faf("just a string".to_string()));
        assert!(!validate_faf("- list\n- items".to_string()));
        assert!(!validate_faf("42".to_string()));
        assert!(!validate_faf("true".to_string()));
    }

    #[test]
    fn test_validate_faf_rejects_broken_yaml() {
        assert!(!validate_faf("[invalid: yaml: {{{".to_string()));
    }

    #[test]
    fn test_score_faf_returns_json_with_score() {
        let result = score_faf("project:\n  name: test".to_string()).unwrap();
        assert!(result.contains("\"score\":"));
        assert!(result.contains("\"tier\":"));
        assert!(result.contains("\"total\":21"));
    }

    #[test]
    fn test_score_faf_enterprise_returns_33_total() {
        let result = score_faf_enterprise("project:\n  name: test".to_string()).unwrap();
        assert!(result.contains("\"total\":33"));
    }

    #[test]
    fn test_compile_decompile_fafb_roundtrip() {
        let yaml = "faf_version: \"1.0\"\nproject_name: test\n".to_string();
        let bytes = compile_fafb(yaml).unwrap();
        assert_eq!(&bytes[0..4], b"FAFB");
        let json = decompile_fafb(&bytes).unwrap();
        assert!(json.contains("\"sections\":"));
    }

    #[test]
    fn test_score_fafb_from_compiled() {
        let yaml = "faf_version: \"1.0\"\nproject_name: test\n".to_string();
        let bytes = compile_fafb(yaml).unwrap();
        let score = score_fafb(&bytes).unwrap();
        assert!(score.contains("\"source\":\"fafb_meta\""));
    }

    #[test]
    fn test_fafb_info_from_compiled() {
        let yaml = "faf_version: \"1.0\"\nproject_name: test\n".to_string();
        let bytes = compile_fafb(yaml).unwrap();
        let info = fafb_info(&bytes).unwrap();
        assert!(info.contains("\"section_count\":"));
        assert!(!info.contains("\"content\":"));
    }
}
