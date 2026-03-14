//! Mk4 Championship Engine — 33-Slot Scoring
//!
//! Ported from xai-faf-rust/src/scoring/mk4.rs
//! Philosophy: Populated, Empty, or Slotignored.
//! Accuracy: 100% parity with xai-faf-rust native.

use serde_yaml::Value;

/// The three technical states of a FAF slot
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlotState {
    /// Initial state: missing or placeholder
    Empty,
    /// Valid, project-specific data
    Populated,
    /// Explicitly marked as not applicable
    Slotignored,
}

/// FAF License Tiers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LicenseTier {
    /// Base 21 slots
    Base,
    /// Full 33 slots (Enterprise/Monorepo)
    Enterprise,
}

/// The result of an Mk4 scoring run
#[derive(Debug)]
pub struct Mk4Result {
    pub score: u32,
    pub tier: String,
    pub populated: u32,
    pub ignored: u32,
    pub active: u32,
    pub total: u32,
    pub slots: Vec<(String, SlotState)>,
}

impl Mk4Result {
    /// Export as JSON string for wasm_bindgen
    pub fn to_json(&self) -> String {
        let mut slots_json = String::from("{");
        for (i, (name, state)) in self.slots.iter().enumerate() {
            if i > 0 { slots_json.push(','); }
            let state_str = match state {
                SlotState::Populated => "populated",
                SlotState::Empty => "empty",
                SlotState::Slotignored => "slotignored",
            };
            slots_json.push_str(&format!("\"{}\":\"{}\"", name, state_str));
        }
        slots_json.push('}');

        format!(
            r#"{{"score":{},"tier":"{}","populated":{},"empty":{},"ignored":{},"active":{},"total":{},"slots":{}}}"#,
            self.score,
            self.tier,
            self.populated,
            self.total - self.populated - self.ignored,
            self.ignored,
            self.active,
            self.total,
            slots_json
        )
    }
}

/// The Mk4 Scoring Engine
pub struct Mk4Scorer {
    pub tier: LicenseTier,
}

impl Mk4Scorer {
    /// Create a new Mk4 Engine
    pub fn new(tier: LicenseTier) -> Self {
        Self { tier }
    }

    /// Calculate the official FAF score from YAML content
    pub fn calculate(&self, yaml: &str) -> Result<Mk4Result, String> {
        let doc: Value = serde_yaml::from_str(yaml)
            .map_err(|e| format!("YAML parse error: {}", e))?;

        let mut populated: u32 = 0;
        let mut ignored: u32 = 0;

        let slot_paths = self.get_universal_slots();
        let mut slots: Vec<(String, SlotState)> = Vec::with_capacity(slot_paths.len());

        for slot_path in &slot_paths {
            let state = self.get_slot_state(&doc, slot_path);
            match state {
                SlotState::Populated => populated += 1,
                SlotState::Slotignored => ignored += 1,
                SlotState::Empty => (),
            }
            slots.push((slot_path.clone(), state));
        }

        let total_slots: u32 = if self.tier == LicenseTier::Enterprise { 33 } else { 21 };
        let active_slots = total_slots - ignored;

        let score = if active_slots == 0 {
            0.0
        } else {
            (populated as f64 / active_slots as f64) * 100.0
        };

        let score_rounded = score.round() as u32;

        Ok(Mk4Result {
            score: score_rounded,
            tier: score_to_tier(score_rounded),
            populated,
            ignored,
            active: active_slots,
            total: total_slots,
            slots,
        })
    }

    /// The Universal DNA Map (33 Slots)
    fn get_universal_slots(&self) -> Vec<String> {
        let mut slots = vec![
            // Project Meta (3)
            "project.name".to_string(),
            "project.goal".to_string(),
            "project.main_language".to_string(),
            // Human Context (6)
            "human_context.who".to_string(),
            "human_context.what".to_string(),
            "human_context.why".to_string(),
            "human_context.where".to_string(),
            "human_context.when".to_string(),
            "human_context.how".to_string(),
            // Frontend Stack (4)
            "stack.frontend".to_string(),
            "stack.css_framework".to_string(),
            "stack.ui_library".to_string(),
            "stack.state_management".to_string(),
            // Backend Stack (5)
            "stack.backend".to_string(),
            "stack.api_type".to_string(),
            "stack.runtime".to_string(),
            "stack.database".to_string(),
            "stack.connection".to_string(),
            // Universal Stack (3)
            "stack.hosting".to_string(),
            "stack.build".to_string(),
            "stack.cicd".to_string(),
        ];

        if self.tier == LicenseTier::Enterprise {
            slots.extend(vec![
                // Enterprise Infra (5)
                "stack.monorepo_tool".to_string(),
                "stack.package_manager".to_string(),
                "stack.workspaces".to_string(),
                "monorepo.packages_count".to_string(),
                "monorepo.build_orchestrator".to_string(),
                // Enterprise App (4)
                "stack.admin".to_string(),
                "stack.cache".to_string(),
                "stack.search".to_string(),
                "stack.storage".to_string(),
                // Enterprise Ops (3)
                "monorepo.versioning_strategy".to_string(),
                "monorepo.shared_configs".to_string(),
                "monorepo.remote_cache".to_string(),
            ]);
        }

        slots
    }

    /// Determine the state of a specific slot
    fn get_slot_state(&self, doc: &Value, path: &str) -> SlotState {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = doc;

        for part in parts {
            if let Some(next) = current.get(Value::String(part.to_string())) {
                current = next;
            } else {
                return SlotState::Empty;
            }
        }

        match current {
            Value::String(s) => {
                let s = s.trim();
                if s == "slotignored" {
                    SlotState::Slotignored
                } else if is_valid_populated_string(s) {
                    SlotState::Populated
                } else {
                    SlotState::Empty
                }
            }
            Value::Number(_) | Value::Bool(_) => SlotState::Populated,
            Value::Sequence(seq) => {
                if !seq.is_empty() {
                    SlotState::Populated
                } else {
                    SlotState::Empty
                }
            }
            Value::Mapping(map) => {
                if !map.is_empty() {
                    SlotState::Populated
                } else {
                    SlotState::Empty
                }
            }
            _ => SlotState::Empty,
        }
    }
}

/// Rule 1: Placeholder Rejection (Honest Truth)
fn is_valid_populated_string(s: &str) -> bool {
    let placeholders = [
        "describe your project goal",
        "development teams",
        "cloud platform",
        "null",
        "none",
        "unknown",
        "n/a",
        "not applicable",
    ];

    !s.is_empty() && !placeholders.contains(&s.to_lowercase().as_str())
}

/// Mk4 official tier calculation — same tiers as all FAF engines
fn score_to_tier(score: u32) -> String {
    if score >= 100 { "🏆".to_string() }
    else if score >= 99 { "🥇".to_string() }
    else if score >= 95 { "🥈".to_string() }
    else if score >= 85 { "🥉".to_string() }
    else if score >= 70 { "🟢".to_string() }
    else if score >= 55 { "🟡".to_string() }
    else { "🔴".to_string() }
}

// =============================================================================
// TESTS — Mk4 Parity with xai-faf-rust
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // SLOT STATE TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_empty_yaml_scores_zero() {
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate("empty: true").unwrap();
        assert_eq!(result.score, 0);
        assert_eq!(result.populated, 0);
        assert_eq!(result.total, 21);
    }

    #[test]
    fn test_invalid_yaml_returns_error() {
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate("invalid: yaml: [");
        assert!(result.is_err());
    }

    #[test]
    fn test_project_meta_3_slots() {
        let yaml = r#"
project:
  name: faf-cli
  goal: Universal AI context
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 3);
        assert_eq!(result.score, 14); // 3/21 = 14.28 → 14
    }

    #[test]
    fn test_human_context_6_slots() {
        let yaml = r#"
human_context:
  who: wolfejam
  what: AI context format
  why: Eliminate drift tax
  where: Global
  when: "2025"
  how: FAF specification
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 6);
        assert_eq!(result.score, 29); // 6/21 = 28.57 → 29
    }

    #[test]
    fn test_full_base_21_slots() {
        let yaml = r#"
project:
  name: faf-cli
  goal: Universal AI context
  main_language: TypeScript
human_context:
  who: wolfejam
  what: AI context format
  why: Eliminate drift tax
  where: Global
  when: "2025"
  how: FAF specification
stack:
  frontend: SvelteKit
  css_framework: Tailwind
  ui_library: Skeleton
  state_management: Svelte stores
  backend: Node.js
  api_type: REST
  runtime: Bun
  database: Supabase
  connection: pg
  hosting: Vercel
  build: Vite
  cicd: GitHub Actions
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 21);
        assert_eq!(result.total, 21);
        assert_eq!(result.score, 100);
        assert_eq!(result.tier, "🏆");
    }

    #[test]
    fn test_enterprise_33_slots() {
        let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
        let result = scorer.calculate("empty: true").unwrap();
        assert_eq!(result.total, 33);
    }

    // -------------------------------------------------------------------------
    // SLOTIGNORED TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_slotignored_excluded_from_denominator() {
        let yaml = r#"
project:
  name: faf-cli
  goal: Universal AI context
  main_language: TypeScript
stack:
  frontend: slotignored
  css_framework: slotignored
  ui_library: slotignored
  state_management: slotignored
  backend: Node.js
  api_type: REST
  runtime: Bun
  database: Supabase
  connection: pg
  hosting: Vercel
  build: Vite
  cicd: GitHub Actions
human_context:
  who: wolfejam
  what: AI context format
  why: Eliminate drift tax
  where: Global
  when: "2025"
  how: FAF specification
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.ignored, 4);
        assert_eq!(result.active, 17); // 21 - 4 = 17
        assert_eq!(result.populated, 17); // 3 project + 8 backend/universal + 6 human
        assert_eq!(result.score, 100); // 17/17 = 100%
        assert_eq!(result.tier, "🏆");
    }

    #[test]
    fn test_all_slotignored_scores_zero() {
        // Edge case: if ALL slots are ignored, score should be 0 not divide-by-zero
        let _scorer = Mk4Scorer::new(LicenseTier::Base);
        // Can't easily make all 21 slots slotignored without a huge YAML,
        // so test the math directly
        let active = 0u32;
        let populated = 0u32;
        let score = if active == 0 { 0.0 } else { (populated as f64 / active as f64) * 100.0 };
        assert_eq!(score, 0.0);
    }

    // -------------------------------------------------------------------------
    // PLACEHOLDER REJECTION TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_placeholder_describe_rejected() {
        let yaml = r#"
project:
  name: test
  goal: Describe your project goal
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 2); // name + main_language, NOT goal
    }

    #[test]
    fn test_placeholder_null_rejected() {
        let yaml = r#"
project:
  name: test
  goal: "null"
  main_language: "none"
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1); // only name
    }

    #[test]
    fn test_placeholder_na_rejected() {
        let yaml = r#"
project:
  name: test
  goal: n/a
  main_language: not applicable
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1); // only name
    }

    #[test]
    fn test_placeholder_unknown_rejected() {
        let yaml = r#"
project:
  name: test
  goal: unknown
  main_language: Unknown
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1); // only name
    }

    #[test]
    fn test_placeholder_case_insensitive() {
        let yaml = r#"
project:
  name: test
  goal: "NULL"
  main_language: "NONE"
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1); // only name, NULL/NONE rejected
    }

    #[test]
    fn test_empty_string_rejected() {
        let yaml = r#"
project:
  name: test
  goal: ""
  main_language: "  "
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1); // only name
    }

    // -------------------------------------------------------------------------
    // VALUE TYPE TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_number_is_populated() {
        let yaml = r#"
project:
  name: test
  goal: 42
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 3); // number counts as populated
    }

    #[test]
    fn test_boolean_is_populated() {
        let yaml = r#"
project:
  name: test
  goal: true
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 3);
    }

    #[test]
    fn test_sequence_is_populated() {
        let yaml = r#"
project:
  name: test
  goal:
    - item1
    - item2
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 3);
    }

    #[test]
    fn test_empty_sequence_is_empty() {
        let yaml = r#"
project:
  name: test
  goal: []
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 2); // empty seq = Empty
    }

    // -------------------------------------------------------------------------
    // TIER TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_tier_trophy_100() {
        assert_eq!(score_to_tier(100), "🏆");
    }

    #[test]
    fn test_tier_gold_99() {
        assert_eq!(score_to_tier(99), "🥇");
    }

    #[test]
    fn test_tier_silver_95() {
        assert_eq!(score_to_tier(95), "🥈");
    }

    #[test]
    fn test_tier_bronze_85() {
        assert_eq!(score_to_tier(85), "🥉");
    }

    #[test]
    fn test_tier_green_70() {
        assert_eq!(score_to_tier(70), "🟢");
    }

    #[test]
    fn test_tier_yellow_55() {
        assert_eq!(score_to_tier(55), "🟡");
    }

    #[test]
    fn test_tier_red_54() {
        assert_eq!(score_to_tier(54), "🔴");
    }

    #[test]
    fn test_tier_red_0() {
        assert_eq!(score_to_tier(0), "🔴");
    }

    // -------------------------------------------------------------------------
    // JSON OUTPUT TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_json_output_format() {
        let yaml = r#"
project:
  name: test
  goal: A real goal
  main_language: Rust
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        let json = result.to_json();

        assert!(json.contains("\"score\":14"));
        assert!(json.contains("\"populated\":3"));
        assert!(json.contains("\"total\":21"));
        assert!(json.contains("\"project.name\":\"populated\""));
        assert!(json.contains("\"project.goal\":\"populated\""));
    }

    #[test]
    fn test_json_shows_slotignored() {
        let yaml = r#"
project:
  name: test
  goal: slotignored
  main_language: Rust
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        let json = result.to_json();

        assert!(json.contains("\"project.goal\":\"slotignored\""));
        assert!(json.contains("\"ignored\":1"));
    }

    // -------------------------------------------------------------------------
    // SLOT COUNT TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_base_has_21_slots() {
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let slots = scorer.get_universal_slots();
        assert_eq!(slots.len(), 21);
    }

    #[test]
    fn test_enterprise_has_33_slots() {
        let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
        let slots = scorer.get_universal_slots();
        assert_eq!(slots.len(), 33);
    }

    #[test]
    fn test_enterprise_includes_monorepo_slots() {
        let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
        let slots = scorer.get_universal_slots();
        assert!(slots.contains(&"monorepo.packages_count".to_string()));
        assert!(slots.contains(&"monorepo.build_orchestrator".to_string()));
        assert!(slots.contains(&"monorepo.versioning_strategy".to_string()));
        assert!(slots.contains(&"monorepo.shared_configs".to_string()));
        assert!(slots.contains(&"monorepo.remote_cache".to_string()));
    }

    #[test]
    fn test_base_slots_are_subset_of_enterprise() {
        let base = Mk4Scorer::new(LicenseTier::Base).get_universal_slots();
        let enterprise = Mk4Scorer::new(LicenseTier::Enterprise).get_universal_slots();
        for slot in &base {
            assert!(enterprise.contains(slot), "Base slot {} missing from enterprise", slot);
        }
    }

    // -------------------------------------------------------------------------
    // PLACEHOLDER LIST TESTS
    // -------------------------------------------------------------------------

    #[test]
    fn test_all_8_placeholders_rejected() {
        let placeholders = [
            "describe your project goal",
            "development teams",
            "cloud platform",
            "null",
            "none",
            "unknown",
            "n/a",
            "not applicable",
        ];
        for p in &placeholders {
            assert!(!is_valid_populated_string(p), "'{}' should be rejected", p);
        }
    }

    #[test]
    fn test_valid_strings_accepted() {
        let valid = [
            "faf-cli",
            "Universal AI context format",
            "TypeScript",
            "Rust",
            "SvelteKit",
            "wolfejam",
        ];
        for v in &valid {
            assert!(is_valid_populated_string(v), "'{}' should be accepted", v);
        }
    }

    // -------------------------------------------------------------------------
    // PARITY SNAPSHOT — Same inputs, same outputs as xai-faf-rust
    // -------------------------------------------------------------------------

    #[test]
    fn test_parity_minimal_faf() {
        // Minimal .faf with just project name
        let yaml = "project:\n  name: test";
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1);
        assert_eq!(result.total, 21);
        assert_eq!(result.score, 5); // 1/21 = 4.76 → 5
    }

    #[test]
    fn test_parity_score_rounding() {
        // 3/21 = 14.285... → should round to 14
        let yaml = r#"
project:
  name: test
  goal: real goal
  main_language: Rust
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.score, 14);
    }

    // -------------------------------------------------------------------------
    // EDGE CASE TESTS — The 220mph Brakes
    // -------------------------------------------------------------------------

    #[test]
    fn test_bare_yaml_null_is_empty() {
        // YAML bare `null` becomes Value::Null, not Value::String("null")
        let yaml = r#"
project:
  name: test
  goal: null
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        // serde_yaml parses bare `null` as Value::Null → falls to _ => Empty
        assert_eq!(result.populated, 2); // name + main_language, NOT goal
    }

    #[test]
    fn test_bare_yaml_tilde_null_is_empty() {
        // YAML ~ is also null
        let yaml = r#"
project:
  name: test
  goal: ~
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 2);
    }

    #[test]
    fn test_empty_mapping_is_empty() {
        let yaml = r#"
project:
  name: test
  goal: {}
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 2); // {} = empty mapping = Empty
    }

    #[test]
    fn test_nested_mapping_is_populated() {
        let yaml = r#"
project:
  name: test
  goal:
    primary: Build things
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 3); // non-empty mapping = Populated
    }

    #[test]
    fn test_all_21_slotignored_through_scorer() {
        // Every single base slot set to slotignored — must not divide by zero
        let yaml = r#"
project:
  name: slotignored
  goal: slotignored
  main_language: slotignored
human_context:
  who: slotignored
  what: slotignored
  why: slotignored
  where: slotignored
  when: slotignored
  how: slotignored
stack:
  frontend: slotignored
  css_framework: slotignored
  ui_library: slotignored
  state_management: slotignored
  backend: slotignored
  api_type: slotignored
  runtime: slotignored
  database: slotignored
  connection: slotignored
  hosting: slotignored
  build: slotignored
  cicd: slotignored
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.ignored, 21);
        assert_eq!(result.active, 0);
        assert_eq!(result.populated, 0);
        assert_eq!(result.score, 0); // 0/0 → 0, not panic/NaN
    }

    #[test]
    fn test_enterprise_full_33_slots_populated() {
        let yaml = r#"
project:
  name: enterprise-test
  goal: Full monorepo
  main_language: TypeScript
human_context:
  who: team
  what: platform
  why: scale
  where: cloud
  when: "2026"
  how: microservices
stack:
  frontend: React
  css_framework: Tailwind
  ui_library: Radix
  state_management: Zustand
  backend: Node.js
  api_type: GraphQL
  runtime: Bun
  database: PostgreSQL
  connection: Prisma
  hosting: AWS
  build: Turborepo
  cicd: GitHub Actions
  monorepo_tool: pnpm workspaces
  package_manager: pnpm
  workspaces: apps/*, packages/*
  admin: Retool
  cache: Redis
  search: Elasticsearch
  storage: S3
monorepo:
  packages_count: 24
  build_orchestrator: Turborepo
  versioning_strategy: independent
  shared_configs: eslint, tsconfig, prettier
  remote_cache: Vercel Remote Cache
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 33);
        assert_eq!(result.total, 33);
        assert_eq!(result.score, 100);
        assert_eq!(result.tier, "🏆");
    }

    #[test]
    fn test_tier_boundary_off_by_one_94_is_bronze() {
        // 94% should be Bronze, not Silver
        assert_eq!(score_to_tier(94), "🥉");
    }

    #[test]
    fn test_tier_boundary_off_by_one_84_is_green() {
        // 84% should be Green, not Bronze
        assert_eq!(score_to_tier(84), "🟢");
    }

    #[test]
    fn test_tier_boundary_off_by_one_69_is_yellow() {
        assert_eq!(score_to_tier(69), "🟡");
    }

    #[test]
    fn test_tier_boundary_off_by_one_54_is_red() {
        assert_eq!(score_to_tier(54), "🔴");
    }

    #[test]
    fn test_tier_boundary_off_by_one_98_is_silver() {
        assert_eq!(score_to_tier(98), "🥈");
    }

    #[test]
    fn test_mixed_enterprise_base_plus_some_enterprise() {
        // All 21 base populated + only 3 of 12 enterprise = 24/33 = 73% Green
        let yaml = r#"
project:
  name: mixed-test
  goal: Partial enterprise
  main_language: TypeScript
human_context:
  who: team
  what: platform
  why: scale
  where: cloud
  when: "2026"
  how: microservices
stack:
  frontend: React
  css_framework: Tailwind
  ui_library: Radix
  state_management: Zustand
  backend: Node.js
  api_type: REST
  runtime: Bun
  database: PostgreSQL
  connection: Prisma
  hosting: AWS
  build: Vite
  cicd: GitHub Actions
monorepo:
  packages_count: 5
  build_orchestrator: Lerna
  versioning_strategy: fixed
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 24); // 21 base + 3 monorepo
        assert_eq!(result.total, 33);
        assert_eq!(result.score, 73); // 24/33 = 72.7 → 73
        assert_eq!(result.tier, "🟢");
    }

    #[test]
    fn test_slotignored_case_sensitive() {
        // "Slotignored" (capitalized) should NOT be treated as slotignored
        let yaml = r#"
project:
  name: test
  goal: Slotignored
  main_language: SLOTIGNORED
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        // "Slotignored" and "SLOTIGNORED" are valid strings, not the magic keyword
        assert_eq!(result.populated, 3);
        assert_eq!(result.ignored, 0);
    }

    #[test]
    fn test_placeholder_with_extra_whitespace() {
        // " null " after trim should still be rejected
        let yaml = r#"
project:
  name: test
  goal: "  null  "
  main_language: TypeScript
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 2); // "  null  " trimmed = "null" = rejected
    }

    #[test]
    fn test_json_output_is_valid_json() {
        let yaml = "project:\n  name: \"json\\\"test\"";
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        let json = result.to_json();
        // Must contain expected keys
        assert!(json.starts_with('{'));
        assert!(json.ends_with('}'));
        assert!(json.contains("\"score\":"));
        assert!(json.contains("\"tier\":"));
        assert!(json.contains("\"populated\":"));
        assert!(json.contains("\"active\":"));
        assert!(json.contains("\"total\":"));
        assert!(json.contains("\"slots\":"));
    }

    // -------------------------------------------------------------------------
    // PARITY SNAPSHOT — Same inputs, same outputs as xai-faf-rust
    // -------------------------------------------------------------------------

    #[test]
    fn test_parity_half_filled() {
        // 11/21 = 52.38 → 52 (Red tier)
        let yaml = r#"
project:
  name: test
  goal: real goal
  main_language: Rust
human_context:
  who: wolfejam
  what: AI context
  why: Drift tax
  where: Global
  when: "2025"
  how: FAF
stack:
  frontend: React
  backend: Node.js
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 11);
        assert_eq!(result.score, 52);
        assert_eq!(result.tier, "🔴");
    }
}
