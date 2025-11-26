//! FAF Types - WASM-safe structs with serde
//!
//! Supports both minimal and extended FAF formats.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

/// Main FAF project DNA structure - flexible to support extended formats
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
#[wasm_bindgen(getter_with_clone)]
pub struct FafDna {
    // Core sections (standard FAF)
    #[wasm_bindgen(skip)]
    pub project: Option<ProjectSection>,
    #[wasm_bindgen(skip)]
    pub instructions: Option<InstructionsSection>,
    #[wasm_bindgen(skip)]
    pub context: Option<ContextSection>,
    #[wasm_bindgen(skip)]
    pub metadata: Option<MetadataSection>,

    // Extended FAF fields (v2.5+)
    #[wasm_bindgen(skip)]
    pub stack: Option<StackSection>,
    #[wasm_bindgen(skip)]
    pub ai_instructions: Option<AiInstructionsSection>,
    #[wasm_bindgen(skip)]
    pub human_context: Option<HumanContextSection>,
    #[wasm_bindgen(skip)]
    pub preferences: Option<PreferencesSection>,
    #[wasm_bindgen(skip)]
    pub state: Option<StateSection>,
    #[wasm_bindgen(skip)]
    pub tags: Option<TagsSection>,
    #[wasm_bindgen(skip)]
    pub instant_context: Option<InstantContextSection>,
    #[wasm_bindgen(skip)]
    pub context_quality: Option<ContextQualitySection>,
    #[wasm_bindgen(skip)]
    pub ai_scoring_details: Option<AiScoringDetailsSection>,

    // Top-level convenience fields
    pub faf_version: Option<String>,
    pub ai_score: Option<String>,
    pub ai_confidence: Option<String>,
    pub score: Option<i32>,
}

#[wasm_bindgen]
impl FafDna {
    /// Get project name (checks multiple locations)
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.project.as_ref().and_then(|p| p.name.clone())
    }

    /// Get project stack (checks project.stack or top-level stack.frontend)
    #[wasm_bindgen(getter)]
    pub fn project_stack(&self) -> Option<String> {
        self.project.as_ref().and_then(|p| p.stack.clone())
            .or_else(|| self.stack.as_ref().and_then(|s| s.frontend.clone()))
    }

    /// Get embedded AI score if present
    #[wasm_bindgen(getter)]
    pub fn embedded_score(&self) -> Option<i32> {
        self.score
            .or_else(|| self.ai_scoring_details.as_ref().and_then(|d| d.ai_score))
            .or_else(|| {
                self.ai_score.as_ref().and_then(|s| {
                    s.trim_end_matches('%').parse().ok()
                })
            })
    }

    /// Export to JSON for JS consumption
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(self)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

// =============================================================================
// CORE SECTIONS (Standard FAF)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProjectSection {
    pub name: Option<String>,
    pub description: Option<String>,
    pub stack: Option<String>,
    pub version: Option<String>,
    pub mission: Option<String>,
    pub repository: Option<String>,
    pub goal: Option<String>,
    pub main_language: Option<String>,
    #[serde(rename = "type")]
    pub project_type: Option<String>,
    pub brand: Option<String>,
    pub revolution: Option<String>,
    pub generated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InstructionsSection {
    pub ai_context: Option<Vec<String>>,
    pub build: Option<BuildConfig>,
    pub constraints: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BuildConfig {
    pub commands: Option<Vec<String>>,
    pub test: Option<String>,
    pub lint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ContextSection {
    pub key_files: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub architecture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MetadataSection {
    pub faf_version: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub last_sync: Option<String>,
    pub tags: Option<Vec<String>>,
    pub last_enhanced: Option<String>,
    pub enhanced_by: Option<String>,
}

// =============================================================================
// EXTENDED SECTIONS (FAF v2.5+)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StackSection {
    pub frontend: Option<String>,
    pub backend: Option<String>,
    pub runtime: Option<String>,
    pub database: Option<String>,
    pub build: Option<String>,
    pub package_manager: Option<String>,
    pub api_type: Option<String>,
    pub hosting: Option<String>,
    pub cicd: Option<String>,
    pub testing: Option<String>,
    pub language: Option<String>,
    pub css_framework: Option<String>,
    pub ui_library: Option<String>,
    pub state_management: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AiInstructionsSection {
    pub priority_order: Option<Vec<String>>,
    pub working_style: Option<WorkingStyle>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct WorkingStyle {
    pub code_first: Option<bool>,
    pub explanations: Option<String>,
    pub quality_bar: Option<String>,
    pub testing: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HumanContextSection {
    pub who: Option<String>,
    pub what: Option<String>,
    pub why: Option<String>,
    #[serde(rename = "where")]
    pub location: Option<String>,
    pub when: Option<String>,
    pub how: Option<String>,
    pub additional_context: Option<String>,
    pub context_score: Option<i32>,
    pub total_prd_score: Option<i32>,
    pub success_rate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PreferencesSection {
    pub quality_bar: Option<String>,
    pub commit_style: Option<String>,
    pub response_style: Option<String>,
    pub explanation_level: Option<String>,
    pub communication: Option<String>,
    pub testing: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StateSection {
    pub phase: Option<String>,
    pub version: Option<String>,
    pub focus: Option<String>,
    pub status: Option<String>,
    pub next_milestone: Option<String>,
    pub blockers: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsSection {
    pub auto_generated: Option<Vec<String>>,
    pub smart_defaults: Option<Vec<String>>,
    pub user_defined: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InstantContextSection {
    pub what_building: Option<String>,
    pub tech_stack: Option<String>,
    pub main_language: Option<String>,
    pub deployment: Option<String>,
    pub key_files: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ContextQualitySection {
    pub slots_filled: Option<String>,
    pub ai_confidence: Option<String>,
    pub handoff_ready: Option<bool>,
    pub missing_context: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AiScoringDetailsSection {
    pub system_date: Option<String>,
    pub slot_based_percentage: Option<i32>,
    pub ai_score: Option<i32>,
    pub total_slots: Option<i32>,
    pub filled_slots: Option<i32>,
    pub scoring_method: Option<String>,
    pub trust_embedded: Option<String>,
}
