//! FAF WASM SDK - AI Context Format for Edge Compute
//!
//! # Usage (JavaScript)
//! ```js
//! import init, { FAF } from '@faf/wasm-sdk';
//!
//! await init();
//! const faf = new FAF(yamlContent);
//! console.log(faf.weighted_score);  // AI-readiness score
//! console.log(faf.tier);            // Tier emoji
//! ```
//!
//! # Performance
//! - Parse: <1ms
//! - Score: <1ms
//! - Total: <2ms (sub-10ms globally via edge)

mod error;
mod scorer;
pub mod generator;

use wasm_bindgen::prelude::*;
use scorer::{FafScore, Mk3Score, score_yaml, score_yaml_mk3};

/// FAF - Main entry point for WASM
#[wasm_bindgen]
pub struct FAF {
    yaml_content: String,
    score: FafScore,
    mk3_score: Mk3Score,
    project_name: Option<String>,
    project_stack: Option<String>,
}

#[wasm_bindgen]
impl FAF {
    /// Create FAF from YAML content
    #[wasm_bindgen(constructor)]
    pub fn new(yaml_content: String) -> Result<FAF, JsValue> {
        let score = score_yaml(&yaml_content)
            .map_err(|e| JsValue::from_str(&e))?;
        let mk3_score = score_yaml_mk3(&yaml_content)
            .map_err(|e| JsValue::from_str(&e))?;

        // Extract project name and stack from YAML
        let (project_name, project_stack) = extract_project_info(&yaml_content);

        Ok(FAF {
            yaml_content,
            score,
            mk3_score,
            project_name,
            project_stack,
        })
    }

    /// Get project name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.project_name.clone()
    }

    /// Get project stack
    #[wasm_bindgen(getter)]
    pub fn stack(&self) -> Option<String> {
        self.project_stack.clone()
    }

    // =========================================================================
    // ELON WEIGHTS (xAI only - compile with --features xai)
    // =========================================================================

    /// Get weighted AI-readiness score (0-100) - xAI only
    #[cfg(feature = "xai")]
    #[wasm_bindgen(getter)]
    pub fn weighted_score(&self) -> f64 {
        self.score.weighted()
    }

    /// Get truth score (unweighted, 0-100) - xAI only
    #[cfg(feature = "xai")]
    #[wasm_bindgen(getter)]
    pub fn truth_score(&self) -> f64 {
        self.score.truth()
    }

    /// Get tier emoji (Elon Weights) - xAI only
    #[cfg(feature = "xai")]
    #[wasm_bindgen(getter)]
    pub fn tier(&self) -> String {
        self.score.tier()
    }

    /// Get completeness score - xAI only
    #[cfg(feature = "xai")]
    #[wasm_bindgen(getter)]
    pub fn completeness(&self) -> f64 {
        self.score.completeness()
    }

    /// Get clarity score - xAI only
    #[cfg(feature = "xai")]
    #[wasm_bindgen(getter)]
    pub fn clarity(&self) -> f64 {
        self.score.clarity()
    }

    /// Get structure score - xAI only
    #[cfg(feature = "xai")]
    #[wasm_bindgen(getter)]
    pub fn structure(&self) -> f64 {
        self.score.structure()
    }

    /// Get metadata score - xAI only
    #[cfg(feature = "xai")]
    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> f64 {
        self.score.metadata()
    }

    // =========================================================================
    // MK3 SLOT-BASED SCORING (Official FAF Standard)
    // =========================================================================

    /// Get Mk3 slot-based score (0-100)
    #[wasm_bindgen(getter)]
    pub fn mk3_score(&self) -> f64 {
        self.mk3_score.score()
    }

    /// Get Mk3 tier emoji
    #[wasm_bindgen(getter)]
    pub fn mk3_tier(&self) -> String {
        self.mk3_score.tier()
    }

    /// Get Mk3 filled slots count
    #[wasm_bindgen(getter)]
    pub fn mk3_filled(&self) -> u32 {
        self.mk3_score.filled()
    }

    /// Get Mk3 total slots count
    #[wasm_bindgen(getter)]
    pub fn mk3_total(&self) -> u32 {
        self.mk3_score.total()
    }

    /// Get Mk3 breakdown string
    pub fn mk3_breakdown(&self) -> String {
        self.mk3_score.breakdown()
    }

    /// Get Mk3 display string
    pub fn mk3_display(&self) -> String {
        self.mk3_score.display()
    }

    /// Get score with language bonus
    pub fn score_with_bonus(&self, language: &str) -> f64 {
        self.score.with_bonus(language)
    }

    /// Get display string
    pub fn display(&self) -> String {
        self.score.display()
    }

    /// Export score as JSON
    pub fn score_json(&self) -> String {
        self.score.to_json()
    }

    /// Validate FAF content (returns true if valid)
    pub fn validate(yaml_content: String) -> bool {
        score_yaml(&yaml_content).is_ok()
    }

    /// Get version
    pub fn version() -> String {
        "1.2.0".to_string()
    }
}

/// Extract project name and stack from YAML
fn extract_project_info(yaml_content: &str) -> (Option<String>, Option<String>) {
    use serde_yaml::Value;

    let doc: Value = match serde_yaml::from_str(yaml_content) {
        Ok(d) => d,
        Err(_) => return (None, None),
    };

    let mut name = None;
    let mut stack = None;

    if let Value::Mapping(map) = &doc {
        // Check project.name or top-level name
        if let Some(project) = map.get(&Value::String("project".to_string())) {
            if let Value::Mapping(pmap) = project {
                if let Some(n) = pmap.get(&Value::String("name".to_string())) {
                    name = n.as_str().map(|s| s.to_string());
                }
            }
        }
        if name.is_none() {
            if let Some(n) = map.get(&Value::String("project_name".to_string())) {
                name = n.as_str().map(|s| s.to_string());
            }
        }
        if name.is_none() {
            if let Some(n) = map.get(&Value::String("projectName".to_string())) {
                name = n.as_str().map(|s| s.to_string());
            }
        }

        // Check stack.frontend or top-level tech_stack
        if let Some(s) = map.get(&Value::String("stack".to_string())) {
            if let Value::Mapping(smap) = s {
                if let Some(fe) = smap.get(&Value::String("frontend".to_string())) {
                    stack = fe.as_str().map(|s| s.to_string());
                }
            }
        }
        if stack.is_none() {
            if let Some(ts) = map.get(&Value::String("tech_stack".to_string())) {
                if let Value::Sequence(seq) = ts {
                    if let Some(first) = seq.first() {
                        stack = first.as_str().map(|s| s.to_string());
                    }
                }
            }
        }
    }

    (name, stack)
}

/// Standalone validate function
#[wasm_bindgen]
pub fn validate_faf(yaml_content: String) -> bool {
    FAF::validate(yaml_content)
}

/// Standalone score function - returns JSON
#[wasm_bindgen]
pub fn score_faf(yaml_content: String) -> Result<String, JsValue> {
    let faf = FAF::new(yaml_content)?;
    Ok(faf.score_json())
}

/// Get SDK version
#[wasm_bindgen]
pub fn sdk_version() -> String {
    "1.2.0".to_string()
}

// =============================================================================
// HOT PATH RE-EXPORTS - xAI/Grok Recommended
// =============================================================================

pub use scorer::{score_weights, score_weights_fast, WEIGHTS_F32};
