//! WJTTC Stress Tests — Tier 1 Brake Systems
//!
//! "We break things so others never have to know they were broken."
//!
//! These tests try to crash, corrupt, or confuse the engine with
//! adversarial inputs. If the brakes fail at 220mph, people die.

#[cfg(test)]
mod stress {
    use faf_wasm_sdk::mk4::{Mk4Scorer, LicenseTier};
    use faf_wasm_sdk::fafb;
    use faf_wasm_sdk::validate_faf;

    // =========================================================================
    // TIER 1: BRAKE SYSTEMS — Must never crash, corrupt, or panic
    // =========================================================================

    #[test]
    fn stress_yaml_bomb_deep_nesting() {
        // 50 levels of nesting — must not stack overflow
        let mut yaml = String::new();
        for i in 0..50 {
            yaml.push_str(&" ".repeat(i * 2));
            yaml.push_str(&format!("level{}:\n", i));
        }
        yaml.push_str(&" ".repeat(100));
        yaml.push_str("value: deep");
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(&yaml);
        // Must not panic — result is ok with score 0 (no FAF slots found)
        assert!(result.is_ok());
        assert_eq!(result.unwrap().score, 0);
    }

    #[test]
    fn stress_yaml_bomb_wide_mapping() {
        // 1000 keys at top level
        let mut yaml = String::new();
        for i in 0..1000 {
            yaml.push_str(&format!("key_{}: value_{}\n", i, i));
        }
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(&yaml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().score, 0); // no FAF slots
    }

    #[test]
    fn stress_very_long_slot_value() {
        // 100KB string in a slot value
        let long_value: String = "a".repeat(100_000);
        let yaml = format!("project:\n  name: {}\n  goal: Real goal\n  main_language: Rust", long_value);
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(&yaml).unwrap();
        assert_eq!(result.populated, 3); // long string is still populated
    }

    #[test]
    fn stress_unicode_slot_names() {
        // Slot paths with unicode won't match FAF slots but must not crash
        let yaml = "プロジェクト:\n  名前: テスト\nproject:\n  name: test";
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1); // only project.name matches
    }

    #[test]
    fn stress_emoji_values() {
        let yaml = r#"
project:
  name: "🏎️ FAF Racing"
  goal: "🏆 Win championships"
  main_language: "🦀 Rust"
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 3);
    }

    #[test]
    fn stress_binary_in_yaml_string() {
        // Null bytes and control characters in values
        let yaml = "project:\n  name: \"test\\x00\\x01\\x02\"";
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        // serde_yaml may reject or accept — must not panic
        let _ = scorer.calculate(yaml);
    }

    #[test]
    fn stress_duplicate_keys() {
        // serde_yaml 0.9 strictly rejects duplicate keys — must be clean error
        let yaml = r#"
project:
  name: first
  name: second
  goal: test
  main_language: Rust
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("duplicate"));
    }

    #[test]
    fn stress_multiline_strings() {
        let yaml = r#"
project:
  name: test
  goal: |
    This is a very long goal
    that spans multiple lines
    and should still count as
    a populated slot value
  main_language: >
    TypeScript with
    some extra text
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 3);
    }

    #[test]
    fn stress_anchor_and_alias() {
        // YAML anchors (&) and aliases (*)
        let yaml = r#"
defaults: &defaults
  frontend: React
  backend: Node.js
project:
  name: anchor-test
  goal: Test anchors
  main_language: TypeScript
stack:
  <<: *defaults
  runtime: Bun
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml);
        // Must not panic — anchors may or may not be supported
        assert!(result.is_ok());
    }

    #[test]
    fn stress_tab_indentation() {
        // Tabs are technically invalid in YAML but must not crash
        let yaml = "project:\n\tname: tab-test\n\tgoal: test";
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml);
        // serde_yaml rejects tabs — must be clean error, not panic
        assert!(result.is_err());
    }

    #[test]
    fn stress_numeric_keys() {
        // Numeric YAML keys (not strings)
        let yaml = "42: answer\nproject:\n  name: test";
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1);
    }

    #[test]
    fn stress_boolean_keys() {
        // YAML booleans as keys
        let yaml = "true: yes\nfalse: no\nproject:\n  name: test";
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let result = scorer.calculate(yaml).unwrap();
        assert_eq!(result.populated, 1);
    }

    // =========================================================================
    // TIER 1: FAFb BRAKE TESTS — Binary format must never corrupt
    // =========================================================================

    #[test]
    fn stress_fafb_compile_large_yaml() {
        // 50KB of YAML content
        let mut yaml = String::from("faf_version: \"1.0\"\nproject:\n  name: stress-test\ntech_stack:\n");
        for i in 0..1000 {
            yaml.push_str(&format!("  - Technology_{}\n", i));
        }
        let result = fafb::compile_fafb(&yaml);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(&bytes[0..4], b"FAFB");

        // Roundtrip must survive
        let json = fafb::decompile_fafb(&bytes).unwrap();
        assert!(json.contains("\"name\":\"faf_version\""));
    }

    #[test]
    fn stress_fafb_random_bytes() {
        // Random-looking bytes that start with FAFB magic
        let mut bytes = b"FAFB".to_vec();
        bytes.extend_from_slice(&[0xFF; 100]);
        let result = fafb::decompile_fafb(&bytes);
        // Must not panic — clean error
        assert!(result.is_err());
    }

    #[test]
    fn stress_fafb_zero_bytes() {
        let bytes = vec![0u8; 100];
        let result = fafb::decompile_fafb(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn stress_fafb_one_byte() {
        let result = fafb::decompile_fafb(&[0x46]); // just 'F'
        assert!(result.is_err());
    }

    #[test]
    fn stress_fafb_exactly_header_size() {
        // Exactly 32 bytes starting with FAFB but garbage after
        let mut bytes = b"FAFB".to_vec();
        bytes.extend_from_slice(&[0x00; 28]);
        let result = fafb::decompile_fafb(&bytes);
        // Either parses (empty sections) or errors — must not panic
        let _ = result;
    }

    #[test]
    fn stress_fafb_unicode_project_name() {
        let yaml = "faf_version: \"1.0\"\nproject:\n  name: \"日本語プロジェクト 🏎️\"";
        let bytes = fafb::compile_fafb(yaml).unwrap();
        let json = fafb::decompile_fafb(&bytes).unwrap();
        assert!(json.contains("\"name\":\"project\""));
    }

    #[test]
    fn stress_fafb_score_garbage() {
        let mut bytes = b"FAFB".to_vec();
        bytes.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
        bytes.extend_from_slice(&[0x00; 100]);
        let result = fafb::score_fafb(&bytes);
        // Must not panic
        let _ = result;
    }

    #[test]
    fn stress_fafb_info_garbage() {
        let result = fafb::fafb_info(&[0x00; 50]);
        // Must not panic
        let _ = result;
    }

    // =========================================================================
    // TIER 1: VALIDATE — Must never crash on any input
    // =========================================================================

    #[test]
    fn stress_validate_null_byte() {
        assert!(!validate_faf("\0".to_string()));
    }

    #[test]
    fn stress_validate_only_whitespace() {
        assert!(!validate_faf("   \n\t\n   ".to_string()));
    }

    #[test]
    fn stress_validate_very_long() {
        // 10K unique keys — large but valid YAML mapping
        let long: String = (0..10_000)
            .map(|i| format!("key_{}: value_{}\n", i, i))
            .collect();
        assert!(validate_faf(long));
    }

    #[test]
    fn stress_validate_all_control_chars() {
        let control: String = (0u8..32).map(|b| b as char).collect();
        // Must not panic
        let _ = validate_faf(control);
    }

    // =========================================================================
    // TIER 2: ENGINE — Scoring accuracy under pressure
    // =========================================================================

    #[test]
    fn stress_score_consistency() {
        // Same input must always produce same score
        let yaml = r#"
project:
  name: consistency-test
  goal: Verify determinism
  main_language: Rust
human_context:
  who: tester
  what: SDK
  why: Quality
"#;
        let scorer = Mk4Scorer::new(LicenseTier::Base);
        let score1 = scorer.calculate(yaml).unwrap().score;
        let score2 = scorer.calculate(yaml).unwrap().score;
        let score3 = scorer.calculate(yaml).unwrap().score;
        assert_eq!(score1, score2);
        assert_eq!(score2, score3);
    }

    #[test]
    fn stress_enterprise_base_parity() {
        // Enterprise with only base slots should score lower than Base
        // (more total slots = lower percentage)
        let yaml = r#"
project:
  name: parity-test
  goal: Check tiers
  main_language: Rust
"#;
        let base = Mk4Scorer::new(LicenseTier::Base).calculate(yaml).unwrap();
        let enterprise = Mk4Scorer::new(LicenseTier::Enterprise).calculate(yaml).unwrap();
        // 3/21 > 3/33
        assert!(base.score > enterprise.score);
        assert_eq!(base.populated, enterprise.populated);
    }

    #[test]
    fn stress_json_output_no_nan_no_infinity() {
        // All-ignored must not produce NaN or Infinity in JSON
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
        let json = result.to_json();
        assert!(!json.contains("NaN"));
        assert!(!json.contains("Infinity"));
        assert!(!json.contains("inf"));
        assert!(json.contains("\"score\":0"));
    }
}
