//! WASM-specific tests

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_parse_basic() {
    let yaml = r#"
project:
  name: test-project
  stack: rust
"#;

    let faf = faf_wasm_sdk::FAF::new(yaml.to_string()).unwrap();
    assert_eq!(faf.name(), Some("test-project".to_string()));
    assert_eq!(faf.stack(), Some("rust".to_string()));
}

#[wasm_bindgen_test]
fn test_scoring() {
    let yaml = r#"
project:
  name: test
  description: A test project
  stack: typescript
  version: 1.0.0
  mission: Test FAF

instructions:
  ai_context:
    - Follow best practices
  build:
    commands:
      - npm build

metadata:
  faf_version: "2.8.0"
  author: test
"#;

    let faf = faf_wasm_sdk::FAF::new(yaml.to_string()).unwrap();
    assert!(faf.weighted_score() > 50.0);
    assert!(faf.completeness() > 60.0);
}

#[wasm_bindgen_test]
fn test_validate() {
    assert!(faf_wasm_sdk::FAF::validate("project:\n  name: test".to_string()));
    assert!(!faf_wasm_sdk::FAF::validate("invalid: [yaml".to_string()));
}

#[wasm_bindgen_test]
fn test_indentation() {
    // Test proper YAML indentation handling
    let yaml = r#"
project:
  name: indent-test
  stack: rust
instructions:
  ai_context:
    - First instruction
    - Second instruction
  build:
    commands:
      - cargo build
      - cargo test
context:
  key_files:
    - src/lib.rs
    - Cargo.toml
"#;

    let faf = faf_wasm_sdk::FAF::new(yaml.to_string()).unwrap();
    assert_eq!(faf.name(), Some("indent-test".to_string()));
    assert!(faf.clarity() > 0.0); // Has ai_context
    assert!(faf.structure() > 0.0); // Has key_files
}
