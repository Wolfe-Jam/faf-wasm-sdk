//! Glass Hood Scoring - Full transparency AI-readiness
//!
//! EXACT MATCH with xai-faf-core scoring logic
//!
//! Weights: [0.40, 0.35, 0.15, 0.10]
//! - 40% Completeness (Core DNA)
//! - 35% Clarity (Instructions)
//! - 15% Structure (Context)
//! - 10% Metadata

use wasm_bindgen::prelude::*;
use serde_yaml::Value;

/// Scoring weights - The Glass Hood standard (Elon Weights)
pub const WEIGHTS: [f64; 4] = [0.40, 0.35, 0.15, 0.10];

/// Weight labels for transparency
pub const WEIGHT_LABELS: [&str; 4] = ["Completeness", "Clarity", "Structure", "Metadata"];

/// Language bonuses
pub const BONUS_LANGUAGES: [(&str, f64); 3] = [
    ("rust", 15.0),
    ("go", 10.0),
    ("typescript", 5.0),
];

/// Mk3 Compiler Engine Tier System (OFFICIAL - DO NOT CHANGE)
/// 100%: Championship | 99%+: Gold | 95%+: Silver | 85%+: Bronze
/// 70%+: Green | 55%+: Yellow | <55%: Red

/// FAF Score result - fully transparent
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FafScore {
    completeness: f64,
    clarity: f64,
    structure: f64,
    metadata: f64,
}

#[wasm_bindgen]
impl FafScore {
    /// Create new score
    pub fn new(completeness: f64, clarity: f64, structure: f64, metadata: f64) -> Self {
        Self { completeness, clarity, structure, metadata }
    }

    /// Get completeness score (0-100)
    #[wasm_bindgen(getter)]
    pub fn completeness(&self) -> f64 {
        self.completeness
    }

    /// Get clarity score (0-100)
    #[wasm_bindgen(getter)]
    pub fn clarity(&self) -> f64 {
        self.clarity
    }

    /// Get structure score (0-100)
    #[wasm_bindgen(getter)]
    pub fn structure(&self) -> f64 {
        self.structure
    }

    /// Get metadata score (0-100)
    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> f64 {
        self.metadata
    }

    /// Calculate weighted score
    pub fn weighted(&self) -> f64 {
        self.completeness * WEIGHTS[0]
            + self.clarity * WEIGHTS[1]
            + self.structure * WEIGHTS[2]
            + self.metadata * WEIGHTS[3]
    }

    /// Get truth score (unweighted average)
    pub fn truth(&self) -> f64 {
        (self.completeness + self.clarity + self.structure + self.metadata) / 4.0
    }

    /// Get tier emoji (Mk3 Compiler Engine - OFFICIAL)
    pub fn tier(&self) -> String {
        self.score_to_tier(self.weighted())
    }

    /// Get truth tier emoji (Mk3 Compiler Engine - OFFICIAL)
    pub fn truth_tier(&self) -> String {
        self.score_to_tier(self.truth())
    }

    /// Mk3 official tier calculation - DO NOT CHANGE
    fn score_to_tier(&self, score: f64) -> String {
        if score >= 100.0 { "🏆".to_string() }
        else if score >= 99.0 { "🥇".to_string() }
        else if score >= 95.0 { "🥈".to_string() }
        else if score >= 85.0 { "🥉".to_string() }
        else if score >= 70.0 { "🟢".to_string() }
        else if score >= 55.0 { "🟡".to_string() }
        else { "🔴".to_string() }
    }

    /// Apply language bonus
    pub fn with_bonus(&self, language: &str) -> f64 {
        let bonus = BONUS_LANGUAGES.iter()
            .find(|(l, _)| l.eq_ignore_ascii_case(language))
            .map(|(_, b)| *b)
            .unwrap_or(0.0);
        (self.weighted() + bonus).min(100.0)
    }

    /// Get full display string
    pub fn display(&self) -> String {
        format!(
            "Truth: {:.0}% {} | Weighted: {:.0}% {}",
            self.truth(), self.truth_tier(),
            self.weighted(), self.tier()
        )
    }

    /// Export as JSON for JS
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"completeness":{},"clarity":{},"structure":{},"metadata":{},"weighted":{},"truth":{},"tier":"{}","truth_tier":"{}"}}"#,
            self.completeness, self.clarity, self.structure, self.metadata,
            self.weighted(), self.truth(), self.tier(), self.truth_tier()
        )
    }
}

// =============================================================================
// HOT PATH FUNCTIONS - xAI/Grok Recommended
// =============================================================================

#[wasm_bindgen]
pub fn score_weights(weights: &[f32], base: f32) -> f32 {
    if weights.len() != 4 {
        return 0.0;
    }
    (weights[0] * base + weights[1] * base + weights[2] * base + weights[3] * base).min(100.0)
}

#[wasm_bindgen]
pub fn score_weights_fast(weights: &[f32], values: &[f32]) -> f32 {
    if weights.len() != 4 || values.len() != 4 {
        return 0.0;
    }
    (weights[0] * values[0]
     + weights[1] * values[1]
     + weights[2] * values[2]
     + weights[3] * values[3]).min(100.0)
}

pub const WEIGHTS_F32: [f32; 4] = [0.40, 0.35, 0.15, 0.10];

// =============================================================================
// MK3 SLOT-BASED SCORING - Official FAF Standard
// =============================================================================

/// Mk3 Score result - slot-based (filled/total)
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Mk3Score {
    filled: u32,
    total: u32,
    project_filled: u32,
    project_total: u32,
    stack_filled: u32,
    stack_total: u32,
    human_filled: u32,
    human_total: u32,
}

#[wasm_bindgen]
impl Mk3Score {
    /// Get percentage score (0-100)
    #[wasm_bindgen(getter)]
    pub fn score(&self) -> f64 {
        if self.total == 0 { return 0.0; }
        (self.filled as f64 / self.total as f64) * 100.0
    }

    /// Get filled slot count
    #[wasm_bindgen(getter)]
    pub fn filled(&self) -> u32 { self.filled }

    /// Get total slot count
    #[wasm_bindgen(getter)]
    pub fn total(&self) -> u32 { self.total }

    /// Get tier emoji (Mk3 official)
    #[wasm_bindgen(getter)]
    pub fn tier(&self) -> String {
        let score = self.score();
        if score >= 100.0 { "🏆".to_string() }
        else if score >= 99.0 { "🥇".to_string() }
        else if score >= 95.0 { "🥈".to_string() }
        else if score >= 85.0 { "🥉".to_string() }
        else if score >= 70.0 { "🟢".to_string() }
        else if score >= 55.0 { "🟡".to_string() }
        else { "🔴".to_string() }
    }

    /// Get breakdown as string
    pub fn breakdown(&self) -> String {
        format!(
            "Project: {}/{} | Stack: {}/{} | Human: {}/{}",
            self.project_filled, self.project_total,
            self.stack_filled, self.stack_total,
            self.human_filled, self.human_total
        )
    }

    /// Display string
    pub fn display(&self) -> String {
        format!("{} {:.0}% ({}/{} slots)", self.tier(), self.score(), self.filled, self.total)
    }
}

/// Score YAML using Mk3 slot-based system
pub fn score_yaml_mk3(yaml_content: &str) -> Result<Mk3Score, String> {
    let doc: Value = serde_yaml::from_str(yaml_content)
        .map_err(|e| format!("YAML parse error: {}", e))?;

    let mut project_filled: u32 = 0;
    let mut project_total: u32 = 3; // name, goal, main_language
    let mut stack_filled: u32 = 0;
    let mut stack_total: u32 = 0;
    let mut human_filled: u32 = 0;
    let human_total: u32 = 6; // who, what, why, where, when, how

    if let Value::Mapping(map) = &doc {
        // Helper to check if value is filled (non-empty)
        let is_filled = |v: &Value| -> bool {
            match v {
                Value::String(s) => !s.trim().is_empty() && s != "null" && s != "~",
                Value::Number(_) | Value::Bool(_) => true,
                Value::Sequence(seq) => !seq.is_empty(),
                Value::Mapping(m) => !m.is_empty(),
                _ => false,
            }
        };

        // Project slots (3 required)
        if let Some(project) = map.get(&Value::String("project".to_string())) {
            if let Value::Mapping(pmap) = project {
                if pmap.get(&Value::String("name".to_string())).map(is_filled).unwrap_or(false) {
                    project_filled += 1;
                }
                if pmap.get(&Value::String("goal".to_string())).map(is_filled).unwrap_or(false) {
                    project_filled += 1;
                }
                if pmap.get(&Value::String("main_language".to_string())).map(is_filled).unwrap_or(false) {
                    project_filled += 1;
                }
            }
        }

        // Stack slots (dynamic based on project type)
        if let Some(stack) = map.get(&Value::String("stack".to_string())) {
            if let Value::Mapping(smap) = stack {
                // Count all stack fields
                for (_, v) in smap.iter() {
                    stack_total += 1;
                    if is_filled(v) {
                        stack_filled += 1;
                    }
                }
            }
        }

        // Human context slots (6 required)
        if let Some(hc) = map.get(&Value::String("human_context".to_string())) {
            if let Value::Mapping(hmap) = hc {
                for key in &["who", "what", "why", "where", "when", "how"] {
                    if hmap.get(&Value::String(key.to_string())).map(is_filled).unwrap_or(false) {
                        human_filled += 1;
                    }
                }
            }
        }
    }

    let total = project_total + stack_total + human_total;
    let filled = project_filled + stack_filled + human_filled;

    Ok(Mk3Score {
        filled,
        total,
        project_filled,
        project_total,
        stack_filled,
        stack_total,
        human_filled,
        human_total,
    })
}

// =============================================================================
// ELON WEIGHTS SCORING - xAI/Grok Optimized
// =============================================================================

/// Score YAML content - mirrors xai-faf-core GlassHoodAnalyzer::analyze_yaml
pub fn score_yaml(yaml_content: &str) -> Result<FafScore, String> {
    let doc: Value = serde_yaml::from_str(yaml_content)
        .map_err(|e| format!("YAML parse error: {}", e))?;

    let mut completeness: f64 = 0.0;
    let mut clarity: f64 = 0.0;
    let mut structure: f64 = 50.0; // Base for valid YAML
    let mut metadata: f64 = 0.0;

    if let Value::Mapping(map) = &doc {
        // Helper to get string value from various field names
        let get_str = |keys: &[&str]| -> Option<String> {
            for key in keys {
                if let Some(val) = map.get(&Value::String(key.to_string())) {
                    if let Some(s) = val.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
            // Also check nested project.* fields
            if let Some(project) = map.get(&Value::String("project".to_string())) {
                if let Value::Mapping(pmap) = project {
                    for key in keys {
                        // Strip "project_" prefix if present
                        let nested_key = key.strip_prefix("project_").unwrap_or(*key);
                        if let Some(val) = pmap.get(&Value::String(nested_key.to_string())) {
                            if let Some(s) = val.as_str() {
                                return Some(s.to_string());
                            }
                        }
                    }
                }
            }
            None
        };

        let get_seq_len = |keys: &[&str]| -> Option<usize> {
            for key in keys {
                if let Some(val) = map.get(&Value::String(key.to_string())) {
                    if let Value::Sequence(seq) = val {
                        return Some(seq.len());
                    }
                }
            }
            // Check nested locations
            for section in &["instant_context", "context", "stack"] {
                if let Some(sec) = map.get(&Value::String(section.to_string())) {
                    if let Value::Mapping(smap) = sec {
                        for key in keys {
                            if let Some(val) = smap.get(&Value::String(key.to_string())) {
                                if let Value::Sequence(seq) = val {
                                    return Some(seq.len());
                                }
                            }
                        }
                    }
                }
            }
            None
        };

        // project_name / project.name / name
        if let Some(name) = get_str(&["project_name", "name", "projectName"]) {
            completeness += 20.0;
            if name.len() > 3 {
                clarity += 10.0;
            }
        }

        // mission / project.mission / goal
        if let Some(mission) = get_str(&["mission", "goal", "description"]) {
            completeness += 20.0;
            if mission.len() > 50 {
                clarity += 25.0;
            } else if mission.len() > 20 {
                clarity += 15.0;
            }
        }

        // tech_stack / stack (as mapping or sequence)
        let has_stack = map.get(&Value::String("tech_stack".to_string())).is_some()
            || map.get(&Value::String("stack".to_string())).is_some();

        if has_stack {
            // Check if stack is a sequence
            if let Some(count) = get_seq_len(&["tech_stack"]) {
                let base = 15.0;
                let bonus = (count as f64 * 2.0).min(10.0);
                completeness += base + bonus;
                clarity += 10.0;
            } else if let Some(stack) = map.get(&Value::String("stack".to_string())) {
                // Stack as mapping (extended FAF)
                if let Value::Mapping(smap) = stack {
                    let stack_fields = smap.len();
                    let base = 15.0;
                    let bonus = (stack_fields as f64 * 2.0).min(10.0);
                    completeness += base + bonus;
                    clarity += 10.0;
                }
            }
        }

        // key_files / instant_context.key_files / context.key_files
        if let Some(count) = get_seq_len(&["key_files"]) {
            let base = 15.0;
            let bonus = (count as f64 * 1.0).min(10.0);
            completeness += base + bonus;
            clarity += 15.0;
        }

        // faf_version (metadata) - check multiple locations
        let has_version = get_str(&["faf_version"]).is_some()
            || map.get(&Value::String("ai_scoring_system".to_string())).is_some();
        if has_version {
            metadata += 40.0;
        }

        // author / human_context.who
        let has_author = get_str(&["author"]).is_some()
            || map.get(&Value::String("human_context".to_string()))
                .and_then(|h| h.get("who"))
                .is_some();
        if has_author {
            metadata += 30.0;
        }

        // license
        if get_str(&["license"]).is_some() {
            metadata += 30.0;
        }

        // ai_instructions / ai_context - adds to clarity
        if map.get(&Value::String("ai_instructions".to_string())).is_some() {
            clarity += 20.0;
        }

        // preferences - adds to clarity
        if map.get(&Value::String("preferences".to_string())).is_some() {
            clarity += 10.0;
        }

        // state - adds to structure
        if map.get(&Value::String("state".to_string())).is_some() {
            structure += 10.0;
        }

        // context_quality - adds to structure and metadata
        if let Some(cq) = map.get(&Value::String("context_quality".to_string())) {
            structure += 15.0;
            if let Value::Mapping(cqmap) = cq {
                if cqmap.get(&Value::String("handoff_ready".to_string()))
                    .and_then(|v| v.as_bool())
                    == Some(true) {
                    metadata += 10.0;
                }
            }
        }

        // tags section - adds to metadata
        if map.get(&Value::String("tags".to_string())).is_some() {
            metadata += 10.0;
        }

        // ai_scoring_details - strong metadata signal
        if map.get(&Value::String("ai_scoring_details".to_string())).is_some() {
            metadata += 20.0;
        }

        // human_context section - strong completeness signal
        if let Some(hc) = map.get(&Value::String("human_context".to_string())) {
            if let Value::Mapping(hcmap) = hc {
                let filled = hcmap.iter()
                    .filter(|(_, v)| v.as_str().map(|s| !s.is_empty()).unwrap_or(false))
                    .count();
                completeness += (filled as f64 * 3.0).min(15.0);
            }
        }

        // instant_context - strong signal
        if map.get(&Value::String("instant_context".to_string())).is_some() {
            completeness += 10.0;
            clarity += 10.0;
        }

        // Structure bonus for having many sections (matches xai-faf-core)
        let section_count = map.len();
        structure += (section_count as f64 * 5.0).min(50.0);
    }

    // Cap scores at 100
    let completeness = completeness.min(100.0);
    let clarity = clarity.min(100.0);
    let structure = structure.min(100.0);
    let metadata = metadata.min(100.0);

    Ok(FafScore::new(completeness, clarity, structure, metadata))
}

// =============================================================================
// CARGO UNIT TESTS - Championship Grade
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // WEIGHTS TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_weights_sum_to_one() {
        let sum: f64 = WEIGHTS.iter().sum();
        assert!((sum - 1.0).abs() < 0.001, "Weights must sum to 1.0, got {}", sum);
    }

    #[test]
    fn test_weights_are_correct() {
        assert_eq!(WEIGHTS[0], 0.40, "Completeness weight");
        assert_eq!(WEIGHTS[1], 0.35, "Clarity weight");
        assert_eq!(WEIGHTS[2], 0.15, "Structure weight");
        assert_eq!(WEIGHTS[3], 0.10, "Metadata weight");
    }

    #[test]
    fn test_weight_labels() {
        assert_eq!(WEIGHT_LABELS[0], "Completeness");
        assert_eq!(WEIGHT_LABELS[1], "Clarity");
        assert_eq!(WEIGHT_LABELS[2], "Structure");
        assert_eq!(WEIGHT_LABELS[3], "Metadata");
    }

    // -------------------------------------------------------------------------
    // TIER TESTS - Mk3 Compiler Engine
    // -------------------------------------------------------------------------

    #[test]
    fn test_tier_trophy() {
        let score = FafScore::new(100.0, 100.0, 100.0, 100.0);
        assert_eq!(score.tier(), "🏆");
    }

    #[test]
    fn test_tier_gold() {
        let score = FafScore::new(99.0, 99.0, 99.0, 99.0);
        assert_eq!(score.tier(), "🥇");
    }

    #[test]
    fn test_tier_silver() {
        let score = FafScore::new(95.0, 95.0, 95.0, 95.0);
        assert_eq!(score.tier(), "🥈");
    }

    #[test]
    fn test_tier_bronze() {
        let score = FafScore::new(85.0, 85.0, 85.0, 85.0);
        assert_eq!(score.tier(), "🥉");
    }

    #[test]
    fn test_tier_green() {
        let score = FafScore::new(70.0, 70.0, 70.0, 70.0);
        assert_eq!(score.tier(), "🟢");
    }

    #[test]
    fn test_tier_yellow() {
        let score = FafScore::new(55.0, 55.0, 55.0, 55.0);
        assert_eq!(score.tier(), "🟡");
    }

    #[test]
    fn test_tier_red() {
        let score = FafScore::new(50.0, 50.0, 50.0, 50.0);
        assert_eq!(score.tier(), "🔴");
    }

    // -------------------------------------------------------------------------
    // WEIGHTED SCORE TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_weighted_calculation() {
        let score = FafScore::new(100.0, 100.0, 100.0, 100.0);
        let weighted = score.weighted();
        assert!((weighted - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_weighted_with_different_values() {
        let score = FafScore::new(80.0, 60.0, 40.0, 20.0);
        // 80*0.40 + 60*0.35 + 40*0.15 + 20*0.10 = 32 + 21 + 6 + 2 = 61
        let weighted = score.weighted();
        assert!((weighted - 61.0).abs() < 0.001, "Expected 61.0, got {}", weighted);
    }

    #[test]
    fn test_truth_score() {
        let score = FafScore::new(80.0, 60.0, 40.0, 20.0);
        // (80 + 60 + 40 + 20) / 4 = 50
        let truth = score.truth();
        assert!((truth - 50.0).abs() < 0.001, "Expected 50.0, got {}", truth);
    }

    // -------------------------------------------------------------------------
    // LANGUAGE BONUS TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_rust_bonus() {
        let score = FafScore::new(80.0, 80.0, 80.0, 80.0);
        let with_bonus = score.with_bonus("rust");
        assert_eq!(with_bonus, 95.0, "Rust adds 15% bonus");
    }

    #[test]
    fn test_go_bonus() {
        let score = FafScore::new(80.0, 80.0, 80.0, 80.0);
        let with_bonus = score.with_bonus("go");
        assert_eq!(with_bonus, 90.0, "Go adds 10% bonus");
    }

    #[test]
    fn test_typescript_bonus() {
        let score = FafScore::new(80.0, 80.0, 80.0, 80.0);
        let with_bonus = score.with_bonus("typescript");
        assert_eq!(with_bonus, 85.0, "TypeScript adds 5% bonus");
    }

    #[test]
    fn test_bonus_caps_at_100() {
        let score = FafScore::new(95.0, 95.0, 95.0, 95.0);
        let with_bonus = score.with_bonus("rust");
        assert_eq!(with_bonus, 100.0, "Score should cap at 100");
    }

    #[test]
    fn test_no_bonus_for_unknown_language() {
        let score = FafScore::new(80.0, 80.0, 80.0, 80.0);
        let with_bonus = score.with_bonus("cobol");
        assert_eq!(with_bonus, 80.0, "Unknown language gets no bonus");
    }

    // -------------------------------------------------------------------------
    // YAML PARSING TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_minimal_yaml() {
        let yaml = "project:\n  name: test";
        let result = score_yaml(yaml);
        assert!(result.is_ok(), "Should parse minimal YAML");
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let yaml = "invalid: yaml: content: [";
        let result = score_yaml(yaml);
        assert!(result.is_err(), "Should reject invalid YAML");
    }

    #[test]
    fn test_parse_full_faf() {
        let yaml = r#"
project:
  name: test-project
  description: A test project
  stack: rust
  version: 1.0.0
  mission: Championship testing

instructions:
  ai_context:
    - Test item 1
    - Test item 2

context:
  key_files:
    - src/main.rs
    - Cargo.toml

metadata:
  faf_version: "2.8.0"
  author: wolfejam
  license: MIT
"#;
        let result = score_yaml(yaml);
        assert!(result.is_ok(), "Should parse full FAF");
        let score = result.unwrap();
        assert!(score.weighted() > 0.0, "Should have positive score");
    }

    // -------------------------------------------------------------------------
    // MK3 SLOT-BASED SCORING TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_mk3_empty_yaml() {
        let yaml = "empty: true";
        let result = score_yaml_mk3(yaml);
        assert!(result.is_ok());
        let mk3 = result.unwrap();
        assert_eq!(mk3.filled(), 0, "Empty FAF should have 0 filled slots");
    }

    #[test]
    fn test_mk3_with_project() {
        let yaml = r#"
project:
  name: test
  goal: Test goal
  main_language: rust
"#;
        let result = score_yaml_mk3(yaml);
        assert!(result.is_ok());
        let mk3 = result.unwrap();
        assert_eq!(mk3.project_filled, 3, "All 3 project slots filled");
    }

    #[test]
    fn test_mk3_tier_calculation() {
        let yaml = r#"
project:
  name: test
  goal: Test goal
  main_language: rust
"#;
        let result = score_yaml_mk3(yaml);
        assert!(result.is_ok());
        let mk3 = result.unwrap();
        assert!(mk3.score() > 0.0, "Should have positive score");
    }

    // -------------------------------------------------------------------------
    // HOT PATH FUNCTION TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_score_weights_valid() {
        let weights: [f32; 4] = [0.40, 0.35, 0.15, 0.10];
        let result = score_weights(&weights, 80.0);
        assert!((result - 80.0).abs() < 0.01, "All same base should equal base");
    }

    #[test]
    fn test_score_weights_invalid_length() {
        let weights: [f32; 3] = [0.40, 0.35, 0.25];
        let result = score_weights(&weights, 80.0);
        assert_eq!(result, 0.0, "Invalid weight count returns 0");
    }

    #[test]
    fn test_score_weights_fast() {
        let weights: [f32; 4] = [0.40, 0.35, 0.15, 0.10];
        let values: [f32; 4] = [100.0, 100.0, 100.0, 100.0];
        let result = score_weights_fast(&weights, &values);
        assert!((result - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_weights_f32_constant() {
        assert_eq!(WEIGHTS_F32[0], 0.40);
        assert_eq!(WEIGHTS_F32[1], 0.35);
        assert_eq!(WEIGHTS_F32[2], 0.15);
        assert_eq!(WEIGHTS_F32[3], 0.10);
    }

    // -------------------------------------------------------------------------
    // JSON OUTPUT TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_json_output() {
        let score = FafScore::new(80.0, 70.0, 60.0, 50.0);
        let json = score.to_json();
        assert!(json.contains("\"completeness\":80"));
        assert!(json.contains("\"clarity\":70"));
        assert!(json.contains("\"structure\":60"));
        assert!(json.contains("\"metadata\":50"));
    }

    #[test]
    fn test_display_output() {
        let score = FafScore::new(80.0, 70.0, 60.0, 50.0);
        let display = score.display();
        assert!(display.contains("Truth:"));
        assert!(display.contains("Weighted:"));
    }
}
