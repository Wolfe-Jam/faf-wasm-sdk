//! Integration Tests — v2.0.0 Pure Function API
//!
//! Tests the public API through the rlib interface.
//! NOTE: Functions returning Result<_, JsValue> cannot be tested on native targets
//! because JsValue::from_str panics outside wasm32. Those functions are tested
//! via their inner modules (mk4, fafb) which don't use JsValue.
//!
//! This file tests the non-JsValue exports and validates the module structure.

#[cfg(test)]
mod tests {
    use faf_wasm_sdk::sdk_version;
    use faf_wasm_sdk::validate_faf;

    // These modules are pub, so we can test their inner functions directly
    use faf_wasm_sdk::mk4::{Mk4Scorer, LicenseTier};
    use faf_wasm_sdk::fafb;

    // =========================================================================
    // SDK VERSION
    // =========================================================================

    #[test]
    fn test_sdk_version() {
        assert_eq!(sdk_version(), "2.0.0");
    }

    // =========================================================================
    // VALIDATE (no JsValue — returns bool)
    // =========================================================================

    #[test]
    fn test_validate_valid_yaml() {
        assert!(validate_faf("project:\n  name: test".to_string()));
    }

    #[test]
    fn test_validate_invalid_yaml() {
        assert!(!validate_faf("invalid: [yaml: {{{".to_string()));
    }

    #[test]
    fn test_validate_scalar_not_mapping() {
        assert!(!validate_faf("just a string".to_string()));
    }

    #[test]
    fn test_validate_sequence_not_mapping() {
        assert!(!validate_faf("- item1\n- item2".to_string()));
    }

    #[test]
    fn test_validate_empty_string() {
        assert!(!validate_faf("".to_string()));
    }

    #[test]
    fn test_validate_number() {
        assert!(!validate_faf("42".to_string()));
    }

    #[test]
    fn test_validate_boolean() {
        assert!(!validate_faf("true".to_string()));
    }

    #[test]
    fn test_validate_nested_mapping() {
        assert!(validate_faf("a:\n  b:\n    c: deep".to_string()));
    }

    // =========================================================================
    // MK4 SCORING (via inner module — no JsValue)
    // =========================================================================

    #[test]
    fn test_score_minimal() {
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate("project:\n  name: test").unwrap();
        assert_eq!(result.score, 5); // 1/21
        assert_eq!(result.total, 21);
    }

    #[test]
    fn test_score_full_base() {
        let yaml = r#"
project:
  name: test
  goal: Real goal
  main_language: Rust
human_context:
  who: wolfejam
  what: SDK
  why: Speed
  where: Global
  when: "2026"
  how: FAF
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
        assert_eq!(result.score, 100);
        assert_eq!(result.tier, "🏆");
    }

    #[test]
    fn test_score_invalid_yaml() {
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate("not: [valid: yaml: {{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_enterprise_33_total() {
        let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
        let result = scorer.calculate("project:\n  name: test").unwrap();
        assert_eq!(result.total, 33);
    }

    #[test]
    fn test_enterprise_with_monorepo() {
        let yaml = r#"
project:
  name: test
  goal: Monorepo
  main_language: TypeScript
monorepo:
  packages_count: 12
  build_orchestrator: Turborepo
  versioning_strategy: independent
  shared_configs: eslint, tsconfig
  remote_cache: Vercel
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Enterprise);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.total, 33);
        assert_eq!(result.populated, 8); // 3 project + 5 monorepo
    }

    // =========================================================================
    // FAFb (via inner module — no JsValue)
    // =========================================================================

    #[test]
    fn test_compile_and_decompile_roundtrip() {
        let yaml = r#"
faf_version: "2.5.0"
project:
  name: roundtrip-test
tech_stack:
  - Rust
  - WASM
"#;
        let bytes = fafb::compile_fafb(yaml).unwrap();
        assert_eq!(&bytes[0..4], b"FAFB");

        let json = fafb::decompile_fafb(&bytes).unwrap();
        assert!(json.contains("\"sections\":"));
        assert!(json.contains("META"));
    }

    #[test]
    fn test_compile_invalid_yaml() {
        let result = fafb::compile_fafb("not: [valid: yaml: {{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_decompile_garbage_bytes() {
        let result = fafb::decompile_fafb(&[0xFF, 0xFE, 0xFD, 0xFC]);
        assert!(result.is_err());
    }

    #[test]
    fn test_score_fafb_returns_meta() {
        let yaml = r#"
faf_version: "2.5.0"
project_name: score-test
"#;
        let bytes = fafb::compile_fafb(yaml).unwrap();
        let score = fafb::score_fafb(&bytes).unwrap();
        assert!(score.contains("\"source\":\"fafb_meta\""));
        assert!(score.contains("\"name\":\"score-test\""));
    }

    #[test]
    fn test_fafb_info_no_content() {
        let yaml = "faf_version: \"1.0\"\nproject_name: info-test";
        let bytes = fafb::compile_fafb(yaml).unwrap();
        let info = fafb::fafb_info(&bytes).unwrap();

        assert!(info.contains("\"section_count\":"));
        assert!(info.contains("\"is_core\":"));
        assert!(!info.contains("\"content\":"));
    }

    #[test]
    fn test_fafb_info_smaller_than_decompile() {
        let yaml = r#"
faf_version: "1.0"
project_name: size-test
tech_stack:
  - Rust
  - TypeScript
  - Python
key_files:
  - path: src/main.rs
    description: Entry point
commands:
  build: cargo build
  test: cargo test
"#;
        let bytes = fafb::compile_fafb(yaml).unwrap();
        let info = fafb::fafb_info(&bytes).unwrap();
        let full = fafb::decompile_fafb(&bytes).unwrap();
        assert!(info.len() < full.len());
    }

    #[test]
    fn test_compile_empty_errors() {
        let result = fafb::compile_fafb("");
        assert!(result.is_err());
    }
}
