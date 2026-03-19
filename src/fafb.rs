//! FAFb Binary Format - WASM wrapper around faf-rust-sdk binary module
//!
//! Provides compile/decompile/info/score functions that accept and return
//! strings (YAML/JSON/base64) for clean WASM interop.

use faf_rust_sdk::binary::{
    compile, decompile, CompileOptions, DecompiledFafb,
};

/// Compile YAML source to FAFb binary bytes (WASM-safe: no SystemTime)
pub fn compile_fafb(yaml: &str) -> Result<Vec<u8>, String> {
    let opts = CompileOptions { use_timestamp: false };
    compile(yaml, &opts)
}

/// Decompile FAFb binary bytes to JSON representation
pub fn decompile_fafb(bytes: &[u8]) -> Result<String, String> {
    let result = decompile(bytes).map_err(|e| e.to_string())?;
    Ok(decompiled_to_json(&result))
}

/// Get FAFb file info as JSON (header + section table)
pub fn fafb_info(bytes: &[u8]) -> Result<String, String> {
    let result = decompile(bytes).map_err(|e| e.to_string())?;
    Ok(info_to_json(&result))
}

/// Score a FAFb binary file (read Mk4 score from meta section if present)
pub fn score_fafb(bytes: &[u8]) -> Result<String, String> {
    let result = decompile(bytes).map_err(|e| e.to_string())?;

    // Extract score from project section, faf_version from its own section
    let faf_version = result
        .get_section_string_by_name("faf_version")
        .unwrap_or_default();
    let project_content = result
        .get_section_string_by_name("project")
        .unwrap_or_default();

    let score_info = extract_score_from_meta(&project_content, &faf_version);
    Ok(score_info)
}

/// Convert DecompiledFafb to JSON with all section data
fn decompiled_to_json(result: &DecompiledFafb) -> String {
    let header = &result.header;
    let sections = &result.section_table;

    let mut json = String::from("{");

    json.push_str(&format!(
        "\"version\":\"{}.{}\",\"flags\":{},\"section_count\":{},\"total_size\":{},\"source_checksum\":\"{:#010x}\",",
        header.version_major,
        header.version_minor,
        header.flags.raw(),
        header.section_count,
        header.total_size,
        header.source_checksum
    ));

    json.push_str("\"sections\":[");
    let entries = sections.entries();
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        let content = result
            .section_string(entry)
            .unwrap_or_default();
        let escaped = escape_json_string(&content);
        let name = result.section_name(entry);
        json.push_str(&format!(
            "{{\"name\":\"{}\",\"type_id\":{},\"priority\":{},\"offset\":{},\"length\":{},\"token_count\":{},\"classification\":\"{}\",\"content\":\"{}\"}}",
            escape_json_string(&name),
            entry.section_type.id(),
            entry.priority.value(),
            entry.offset,
            entry.length,
            entry.token_count,
            entry.classification().name(),
            escaped
        ));
    }
    json.push_str("]}");

    json
}

/// Convert DecompiledFafb to info-only JSON (no section content)
fn info_to_json(result: &DecompiledFafb) -> String {
    let header = &result.header;
    let sections = &result.section_table;

    let mut json = String::from("{");

    json.push_str(&format!(
        "\"version\":\"{}.{}\",\"flags\":{},\"section_count\":{},\"total_size\":{},\"source_checksum\":\"{:#010x}\",\"created\":{},",
        header.version_major,
        header.version_minor,
        header.flags.raw(),
        header.section_count,
        header.total_size,
        header.source_checksum,
        header.created_timestamp
    ));

    json.push_str("\"sections\":[");
    let entries = sections.entries();
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        let name = result.section_name(entry);
        let classification = entry.classification();
        json.push_str(&format!(
            "{{\"name\":\"{}\",\"type_id\":{},\"priority\":{},\"length\":{},\"token_count\":{},\"classification\":\"{}\"}}",
            escape_json_string(&name),
            entry.section_type.id(),
            entry.priority.value(),
            entry.length,
            entry.token_count,
            classification.name()
        ));
    }
    json.push_str("]}");

    json
}

/// Extract score from meta section YAML content
fn extract_score_from_meta(project_yaml: &str, faf_version: &str) -> String {
    use serde_yaml::Value;

    let doc: Value = match serde_yaml::from_str(project_yaml) {
        Ok(d) => d,
        Err(_) => {
            return "{\"score\":null,\"source\":\"fafb_meta\",\"error\":\"no meta section or invalid YAML\"}"
                .to_string();
        }
    };

    let mut json = String::from("{\"source\":\"fafb_meta\"");

    if let Value::Mapping(map) = &doc {
        if let Some(score) = map.get(Value::String("score".to_string())) {
            if let Some(n) = score.as_f64() {
                json.push_str(&format!(",\"score\":{}", n));
            } else if let Some(n) = score.as_u64() {
                json.push_str(&format!(",\"score\":{}", n));
            }
        }
        if let Some(name) = map.get(Value::String("name".to_string())) {
            if let Some(s) = name.as_str() {
                json.push_str(&format!(",\"name\":\"{}\"", escape_json_string(s)));
            }
        }
        if let Some(tier) = map.get(Value::String("tier".to_string())) {
            if let Some(s) = tier.as_str() {
                json.push_str(&format!(",\"tier\":\"{}\"", escape_json_string(s)));
            }
        }
    }

    if !faf_version.is_empty() {
        json.push_str(&format!(",\"faf_version\":\"{}\"", escape_json_string(faf_version)));
    }

    json.push('}');
    json
}

/// Escape a string for JSON embedding
fn escape_json_string(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_FAF: &str = r#"
faf_version: "1.0"
project:
  name: "test-project"
tech_stack:
  - Rust
  - WASM
key_files:
  - path: src/main.rs
    description: Entry point
commands:
  build: cargo build
  test: cargo test
"#;

    #[test]
    fn test_compile_valid_yaml() {
        let result = compile_fafb(MINIMAL_FAF);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(&bytes[0..4], b"FAFB");
    }

    #[test]
    fn test_compile_invalid_yaml() {
        let result = compile_fafb("not: [valid: yaml: {{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_empty_yaml_doc() {
        let result = compile_fafb("---\n");
        // "---\n" parses as Null, not a mapping — should error
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_empty_string_errors() {
        let result = compile_fafb("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_roundtrip_compile_decompile() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let json = decompile_fafb(&bytes).unwrap();

        assert!(json.contains("\"version\":"));
        assert!(json.contains("\"sections\":"));
        assert!(json.contains("\"section_count\":"));
    }

    #[test]
    fn test_decompile_invalid_bytes() {
        let result = decompile_fafb(&[0, 1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_decompile_empty_bytes() {
        let result = decompile_fafb(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_fafb_info() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let info = fafb_info(&bytes).unwrap();

        assert!(info.contains("\"version\":"));
        assert!(info.contains("\"section_count\":"));
        assert!(info.contains("\"total_size\":"));
        assert!(info.contains("\"classification\":"));
        // Info should NOT contain full content
        assert!(!info.contains("\"content\":"));
    }

    #[test]
    fn test_score_fafb() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let score = score_fafb(&bytes).unwrap();

        assert!(score.contains("\"source\":\"fafb_meta\""));
    }

    #[test]
    fn test_compile_preserves_sections() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let json = decompile_fafb(&bytes).unwrap();

        // Should have named sections
        assert!(json.contains("faf_version"));
        assert!(json.contains("project"));
    }

    #[test]
    fn test_fafb_magic_bytes() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        assert!(bytes.len() >= 32);
        assert_eq!(bytes[0], b'F');
        assert_eq!(bytes[1], b'A');
        assert_eq!(bytes[2], b'F');
        assert_eq!(bytes[3], b'B');
    }

    #[test]
    fn test_decompile_sections_have_names() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let json = decompile_fafb(&bytes).unwrap();

        assert!(json.contains("\"name\":"));
        assert!(json.contains("\"priority\":"));
        assert!(json.contains("\"token_count\":"));
        assert!(json.contains("\"classification\":"));
    }

    #[test]
    fn test_info_vs_decompile() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let info = fafb_info(&bytes).unwrap();
        let full = decompile_fafb(&bytes).unwrap();

        assert!(info.len() < full.len());
    }

    #[test]
    fn test_escape_json_string() {
        assert_eq!(escape_json_string("hello"), "hello");
        assert_eq!(escape_json_string("he\"llo"), "he\\\"llo");
        assert_eq!(escape_json_string("line\nnew"), "line\\nnew");
        assert_eq!(escape_json_string("tab\there"), "tab\\there");
        assert_eq!(escape_json_string("back\\slash"), "back\\\\slash");
    }

    #[test]
    fn test_score_meta_parsing() {
        let project = r#"
name: "test-project"
score: 85
tier: "🥉"
"#;
        let result = extract_score_from_meta(project, "1.0");
        assert!(result.contains("\"score\":85"));
        assert!(result.contains("\"name\":\"test-project\""));
        assert!(result.contains("\"faf_version\":\"1.0\""));
    }

    #[test]
    fn test_score_meta_invalid() {
        let result = extract_score_from_meta("---\n- :\n  {{{\n  }}}:", "");
        assert!(result.contains("\"error\":") || result.contains("\"source\":\"fafb_meta\""));
    }

    #[test]
    fn test_score_meta_empty() {
        let result = extract_score_from_meta("", "");
        assert!(result.contains("\"source\":\"fafb_meta\""));
    }

    // ─── Edge cases ───

    #[test]
    fn test_crc32_checksum_consistency() {
        let yaml = "faf_version: \"1.0\"\nproject:\n  name: checksum-test\n";
        let bytes1 = compile_fafb(yaml).unwrap();
        let bytes2 = compile_fafb(yaml).unwrap();
        assert_eq!(bytes1, bytes2); // Deterministic (no timestamp)
    }

    #[test]
    fn test_different_input_different_checksum() {
        let bytes1 = compile_fafb("faf_version: \"1.0\"\nproject:\n  name: aaa\n").unwrap();
        let bytes2 = compile_fafb("faf_version: \"1.0\"\nproject:\n  name: bbb\n").unwrap();
        assert_ne!(bytes1, bytes2);
    }

    #[test]
    fn test_unicode_content_roundtrip() {
        let yaml = r#"
faf_version: "1.0"
project:
  name: "日本語テスト"
tech_stack:
  - "Zig ⚡"
  - "Rust 🦀"
context:
  notes: |
    This project uses émojis and ünïcödé characters.
    Chinese: 中文测试
    Arabic: اختبار
    Math: ∑∫∂
"#;
        let bytes = compile_fafb(yaml).unwrap();
        let json = decompile_fafb(&bytes).unwrap();
        assert!(json.contains("faf_version"));
        assert!(json.contains("sections"));
    }

    #[test]
    fn test_meta_only_compile() {
        let yaml = "faf_version: \"2.5.0\"\n";
        let bytes = compile_fafb(yaml).unwrap();
        assert_eq!(&bytes[0..4], b"FAFB");
        let info = fafb_info(&bytes).unwrap();
        // 1 content section (faf_version) + 1 string table = 2
        assert!(info.contains("\"section_count\":2"));
    }

    #[test]
    fn test_all_section_types() {
        let yaml = r#"
faf_version: "2.5.0"
project:
  name: "all-sections"
tech_stack:
  - Rust
key_files:
  - path: src/main.rs
    description: Entry
commands:
  build: cargo build
architecture: |
  Monolithic design
context: |
  Extra context here
"#;
        let bytes = compile_fafb(yaml).unwrap();
        let json = decompile_fafb(&bytes).unwrap();
        // Sections now referenced by name
        assert!(json.contains("faf_version"));
        assert!(json.contains("tech_stack"));
        assert!(json.contains("key_files"));
        assert!(json.contains("commands"));
        assert!(json.contains("architecture"));
        assert!(json.contains("context"));
    }

    #[test]
    fn test_decompile_truncated_header() {
        let short_bytes = b"FAFB";
        let result = decompile_fafb(short_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_score_fafb_with_embedded_score() {
        let yaml = r#"
faf_version: "2.5.0"
project:
  name: "scored"
  score: 95
  tier: "🥈"
"#;
        let bytes = compile_fafb(yaml).unwrap();
        let score_json = score_fafb(&bytes).unwrap();
        assert!(score_json.contains("\"source\":\"fafb_meta\""));
        assert!(score_json.contains("\"name\":\"scored\""));
    }

    #[test]
    fn test_escape_json_control_chars() {
        assert_eq!(escape_json_string("\x00"), "\\u0000");
        assert_eq!(escape_json_string("\x1f"), "\\u001f");
        assert_eq!(escape_json_string("\r\n"), "\\r\\n");
    }

    #[test]
    fn test_escape_json_mixed() {
        let input = "line1\nline2\t\"quoted\"\\back";
        let escaped = escape_json_string(input);
        assert_eq!(escaped, "line1\\nline2\\t\\\"quoted\\\"\\\\back");
    }

    #[test]
    fn test_large_yaml_roundtrip() {
        let yaml = format!(
            r#"
faf_version: "1.0"
project:
  name: "large-test"
tech_stack:
  - Rust
  - TypeScript
  - Python
key_files:
{}
commands:
  build: cargo build
  test: cargo test
  lint: cargo clippy
context:
  notes: |
    This is a large context block with lots of text.
    It should survive the compile/decompile roundtrip.
    Multiple lines are important for testing.
"#,
            (0..20)
                .map(|i| format!("  - path: src/file_{}.rs\n    description: File {}", i, i))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let bytes = compile_fafb(&yaml).unwrap();
        let json = decompile_fafb(&bytes).unwrap();
        assert!(json.contains("\"section_count\":"));
    }

    #[test]
    fn test_custom_fields_preserved() {
        let yaml = r#"
faf_version: "2.5.0"
project:
  name: "custom-test"
my_custom_data:
  important: true
compliance:
  framework: SOC2
"#;
        let bytes = compile_fafb(yaml).unwrap();
        let json = decompile_fafb(&bytes).unwrap();
        assert!(json.contains("my_custom_data"));
        assert!(json.contains("compliance"));
    }

    #[test]
    fn test_classification_in_json() {
        let yaml = r#"
faf_version: "2.5.0"
project:
  name: "classify-test"
docs:
  readme: README.md
custom_thing:
  data: value
"#;
        let bytes = compile_fafb(yaml).unwrap();
        let json = decompile_fafb(&bytes).unwrap();
        assert!(json.contains("\"classification\":\"DNA\""));
        assert!(json.contains("\"classification\":\"Pointer\""));
        assert!(json.contains("\"classification\":\"Context\""));
    }
}
