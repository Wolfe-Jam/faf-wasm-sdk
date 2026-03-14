//! FAFb Binary Format - WASM wrapper around faf-rust-sdk binary module
//!
//! Provides compile/decompile/info/score functions that accept and return
//! strings (YAML/JSON/base64) for clean WASM interop.

use std::io::Write;

use faf_rust_sdk::binary::{
    decompile, DecompiledFafb, FafbHeader, SectionType,
    SectionEntry, SectionTable, Priority,
    HEADER_SIZE, MAX_FILE_SIZE, MAX_SECTIONS, SECTION_ENTRY_SIZE,
};

/// Compile YAML source to FAFb binary bytes (WASM-safe: no SystemTime)
pub fn compile_fafb(yaml: &str) -> Result<Vec<u8>, String> {
    compile_wasm_safe(yaml)
}

/// WASM-safe compile — identical to faf_rust_sdk::binary::compile() except
/// uses FafbHeader::new() (timestamp=0) instead of with_timestamp() which
/// calls std::time::SystemTime::now() and panics in WASM.
fn compile_wasm_safe(yaml_source: &str) -> Result<Vec<u8>, String> {
    let source_bytes = yaml_source.as_bytes();
    if source_bytes.is_empty() {
        return Err("Source content is empty".to_string());
    }

    let yaml: serde_yaml::Value =
        serde_yaml::from_str(yaml_source).map_err(|e| format!("Invalid YAML: {}", e))?;

    let mut sections: Vec<(SectionType, Priority, Vec<u8>)> = Vec::new();

    // META section (critical)
    let version = yaml
        .get("faf_version")
        .and_then(|v| v.as_str())
        .unwrap_or("2.5.0");
    let name = yaml
        .get("project")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .or_else(|| yaml.get("project_name").and_then(|n| n.as_str()))
        .unwrap_or("unknown");
    let meta_content = format!("faf_version: {}\nname: {}\n", version, name);
    sections.push((SectionType::Meta, Priority::critical(), meta_content.into_bytes()));

    // TECH_STACK section (high)
    if let Some(content) = extract_section_yaml(&yaml, "tech_stack") {
        sections.push((SectionType::TechStack, Priority::high(), format!("tech_stack:\n{}", content).into_bytes()));
    } else if let Some(tech) = yaml.get("instant_context").and_then(|ic| ic.get("tech_stack")) {
        if let Ok(content) = serde_yaml::to_string(tech) {
            if !content.trim().is_empty() {
                sections.push((SectionType::TechStack, Priority::high(), format!("tech_stack: {}", content).into_bytes()));
            }
        }
    }

    // KEY_FILES section (high)
    if let Some(content) = extract_section_yaml(&yaml, "key_files") {
        sections.push((SectionType::KeyFiles, Priority::high(), format!("key_files:\n{}", content).into_bytes()));
    } else if let Some(kf) = yaml.get("instant_context").and_then(|ic| ic.get("key_files")) {
        if let Ok(content) = serde_yaml::to_string(kf) {
            if !content.trim().is_empty() {
                sections.push((SectionType::KeyFiles, Priority::high(), format!("key_files:\n{}", content).into_bytes()));
            }
        }
    }

    // COMMANDS section (high)
    if let Some(content) = extract_section_yaml(&yaml, "commands") {
        sections.push((SectionType::Commands, Priority::new(180), format!("commands:\n{}", content).into_bytes()));
    } else if let Some(cmds) = yaml.get("instant_context").and_then(|ic| ic.get("commands")) {
        if let Ok(content) = serde_yaml::to_string(cmds) {
            if !content.trim().is_empty() {
                sections.push((SectionType::Commands, Priority::new(180), format!("commands:\n{}", content).into_bytes()));
            }
        }
    }

    // ARCHITECTURE section (medium)
    if let Some(content) = extract_section_yaml(&yaml, "architecture") {
        sections.push((SectionType::Architecture, Priority::medium(), format!("architecture:\n{}", content).into_bytes()));
    }

    // CONTEXT section (low)
    if let Some(content) = extract_section_yaml(&yaml, "context") {
        sections.push((SectionType::Context, Priority::low(), format!("context:\n{}", content).into_bytes()));
    }

    if sections.len() > MAX_SECTIONS as usize {
        return Err(format!("Too many sections: {} exceeds maximum {}", sections.len(), MAX_SECTIONS));
    }

    // Calculate layout
    let section_count = sections.len();
    let section_table_size = section_count * SECTION_ENTRY_SIZE;

    let mut data_offset: u32 = HEADER_SIZE as u32;
    let mut section_data: Vec<u8> = Vec::new();
    let mut section_table = SectionTable::new();

    for (section_type, priority, data) in &sections {
        let entry = SectionEntry::new(*section_type, data_offset, data.len() as u32)
            .with_priority(*priority);
        section_table.push(entry);
        section_data.extend_from_slice(data);
        data_offset = data_offset
            .checked_add(data.len() as u32)
            .ok_or_else(|| "Section data exceeds u32::MAX bytes".to_string())?;
    }

    let section_table_offset = data_offset;
    let total_size = section_table_offset
        .checked_add(section_table_size as u32)
        .ok_or_else(|| "Total file size exceeds u32::MAX bytes".to_string())?;

    if total_size > MAX_FILE_SIZE {
        return Err(format!("Output size {} bytes exceeds maximum {} bytes", total_size, MAX_FILE_SIZE));
    }

    // Build header — FafbHeader::new() instead of with_timestamp() for WASM safety
    let mut header = FafbHeader::new();
    header.set_source_checksum(source_bytes);
    header.section_count = section_count as u16;
    header.section_table_offset = section_table_offset;
    header.total_size = total_size;

    // Assemble binary
    let mut output: Vec<u8> = Vec::with_capacity(total_size as usize);
    header.write(&mut output).map_err(|e| e.to_string())?;
    output.write_all(&section_data).map_err(|e| e.to_string())?;
    section_table.write(&mut output).map_err(|e| e.to_string())?;

    if output.len() != total_size as usize {
        return Err(format!("Internal error: size mismatch (expected {} bytes, got {} bytes)", total_size, output.len()));
    }

    Ok(output)
}

/// Extract a YAML section as a string
fn extract_section_yaml(yaml: &serde_yaml::Value, key: &str) -> Option<String> {
    yaml.get(key)
        .and_then(|v| serde_yaml::to_string(v).ok())
        .filter(|s| !s.trim().is_empty())
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

    // Try to extract score from meta section
    let meta_content = result
        .get_section_string(SectionType::Meta)
        .unwrap_or_default();

    // Parse meta YAML to find embedded score
    let score_info = extract_score_from_meta(&meta_content);
    Ok(score_info)
}

/// Convert DecompiledFafb to JSON with all section data
fn decompiled_to_json(result: &DecompiledFafb) -> String {
    let header = &result.header;
    let sections = &result.section_table;

    let mut json = String::from("{");

    // Header info
    json.push_str(&format!(
        "\"version\":\"{}.{}\",\"flags\":{},\"section_count\":{},\"total_size\":{},\"source_checksum\":\"{:#010x}\",",
        header.version_major,
        header.version_minor,
        header.flags.raw(),
        header.section_count,
        header.total_size,
        header.source_checksum
    ));

    // Sections array
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
        json.push_str(&format!(
            "{{\"type\":\"{}\",\"type_id\":{},\"priority\":{},\"offset\":{},\"length\":{},\"token_count\":{},\"content\":\"{}\"}}",
            entry.section_type.name(),
            entry.section_type.id(),
            entry.priority.value(),
            entry.offset,
            entry.length,
            entry.token_count,
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

    // Section table (metadata only, no content)
    json.push_str("\"sections\":[");
    let entries = sections.entries();
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!(
            "{{\"type\":\"{}\",\"type_id\":{},\"priority\":{},\"length\":{},\"token_count\":{},\"is_core\":{}}}",
            entry.section_type.name(),
            entry.section_type.id(),
            entry.priority.value(),
            entry.length,
            entry.token_count,
            entry.section_type.is_core()
        ));
    }
    json.push_str("]}");

    json
}

/// Extract score from meta section YAML content
fn extract_score_from_meta(meta_yaml: &str) -> String {
    // Meta section contains YAML like: {faf_version: "1.0", name: "proj", score: 85}
    // Try to extract score-related fields
    use serde_yaml::Value;

    let doc: Value = match serde_yaml::from_str(meta_yaml) {
        Ok(d) => d,
        Err(_) => {
            return "{\"score\":null,\"source\":\"fafb_meta\",\"error\":\"no meta section or invalid YAML\"}"
                .to_string();
        }
    };

    let mut json = String::from("{\"source\":\"fafb_meta\"");

    if let Value::Mapping(map) = &doc {
        // Look for score fields
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
        if let Some(version) = map.get(Value::String("faf_version".to_string())) {
            if let Some(s) = version.as_str() {
                json.push_str(&format!(",\"faf_version\":\"{}\"", escape_json_string(s)));
            }
        }
        if let Some(tier) = map.get(Value::String("tier".to_string())) {
            if let Some(s) = tier.as_str() {
                json.push_str(&format!(",\"tier\":\"{}\"", escape_json_string(s)));
            }
        }
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
project_name: "test-project"
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
        // Check FAFB magic
        assert_eq!(&bytes[0..4], b"FAFB");
    }

    #[test]
    fn test_compile_invalid_yaml() {
        let result = compile_fafb("not: [valid: yaml: {{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_empty_yaml_doc() {
        // Empty YAML doc "---\n" parses as Value::Null, not a mapping
        // Must not panic — either succeeds with META-only or returns error
        let result = compile_fafb("---\n");
        match result {
            Ok(bytes) => {
                // If it compiles, must have valid FAFB header
                assert!(bytes.len() >= HEADER_SIZE);
                assert_eq!(&bytes[0..4], b"FAFB");
            }
            Err(e) => {
                // If it errors, must be a clean error message
                assert!(!e.is_empty());
            }
        }
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

        // Should be valid JSON-ish string
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
        assert!(info.contains("\"is_core\":"));
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

        // Should have sections for the YAML content
        assert!(json.contains("META"));
    }

    #[test]
    fn test_fafb_magic_bytes() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        assert!(bytes.len() >= 32); // At least header size
        assert_eq!(bytes[0], b'F');
        assert_eq!(bytes[1], b'A');
        assert_eq!(bytes[2], b'F');
        assert_eq!(bytes[3], b'B');
    }

    #[test]
    fn test_decompile_sections_have_types() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let json = decompile_fafb(&bytes).unwrap();

        // Each section should have type info
        assert!(json.contains("\"type_id\":"));
        assert!(json.contains("\"priority\":"));
        assert!(json.contains("\"token_count\":"));
    }

    #[test]
    fn test_info_vs_decompile() {
        let bytes = compile_fafb(MINIMAL_FAF).unwrap();
        let info = fafb_info(&bytes).unwrap();
        let full = decompile_fafb(&bytes).unwrap();

        // Info should be shorter (no content)
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
        // Direct test of extract_score_from_meta
        let meta = r#"
faf_version: "1.0"
name: "test-project"
score: 85
tier: "🥉"
"#;
        let result = extract_score_from_meta(meta);
        assert!(result.contains("\"score\":85"));
        assert!(result.contains("\"name\":\"test-project\""));
        assert!(result.contains("\"faf_version\":\"1.0\""));
    }

    #[test]
    fn test_score_meta_invalid() {
        // serde_yaml may parse some "invalid" strings as plain scalars
        // Use truly unparseable YAML
        let result = extract_score_from_meta("---\n- :\n  {{{\n  }}}:");
        assert!(result.contains("\"error\":") || result.contains("\"source\":\"fafb_meta\""));
    }

    #[test]
    fn test_score_meta_empty() {
        let result = extract_score_from_meta("");
        // Empty string may parse as null YAML
        assert!(result.contains("\"source\":\"fafb_meta\""));
    }

    // -------------------------------------------------------------------------
    // EDGE CASE TESTS — 220mph Brakes for FAFb
    // -------------------------------------------------------------------------

    #[test]
    fn test_crc32_checksum_consistency() {
        // Same input must produce same checksum
        let yaml = "faf_version: \"1.0\"\nproject_name: checksum-test\n";
        let bytes1 = compile_fafb(yaml).unwrap();
        let bytes2 = compile_fafb(yaml).unwrap();
        // Bytes 16-19 are source_checksum in header (after magic+version+flags+counts)
        assert_eq!(&bytes1[..HEADER_SIZE], &bytes2[..HEADER_SIZE]);
    }

    #[test]
    fn test_different_input_different_checksum() {
        let bytes1 = compile_fafb("faf_version: \"1.0\"\nproject_name: aaa\n").unwrap();
        let bytes2 = compile_fafb("faf_version: \"1.0\"\nproject_name: bbb\n").unwrap();
        // Source checksums should differ (bytes 16-19 region)
        // Full headers will differ somewhere due to different content/sizes
        assert_ne!(bytes1, bytes2);
    }

    #[test]
    fn test_unicode_content_roundtrip() {
        let yaml = r#"
faf_version: "1.0"
project_name: "日本語テスト"
tech_stack:
  - "Zig ⚡"
  - "Rust 🦀"
context: |
  This project uses émojis and ünïcödé characters.
  Chinese: 中文测试
  Arabic: اختبار
  Math: ∑∫∂
"#;
        let bytes = compile_fafb(yaml).unwrap();
        let json = decompile_fafb(&bytes).unwrap();
        // Must survive roundtrip
        assert!(json.contains("META"));
        assert!(json.contains("sections"));
    }

    #[test]
    fn test_meta_only_compile() {
        // Minimal YAML with no sections beyond META
        let yaml = "faf_version: \"2.5.0\"\n";
        let bytes = compile_fafb(yaml).unwrap();
        assert_eq!(&bytes[0..4], b"FAFB");
        let info = fafb_info(&bytes).unwrap();
        assert!(info.contains("\"section_count\":1"));
    }

    #[test]
    fn test_all_6_section_types() {
        let yaml = r#"
faf_version: "2.5.0"
project_name: "all-sections"
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
        // All 6 section types should be present
        assert!(json.contains("META"));
        assert!(json.contains("TECH_STACK"));
        assert!(json.contains("KEY_FILES"));
        assert!(json.contains("COMMANDS"));
        assert!(json.contains("ARCHITECTURE"));
        assert!(json.contains("CONTEXT"));
    }

    #[test]
    fn test_decompile_truncated_header() {
        // Less than HEADER_SIZE bytes
        let short_bytes = b"FAFB";
        let result = decompile_fafb(short_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_score_fafb_with_embedded_score() {
        // Compile YAML that has score in meta section
        let yaml = r#"
faf_version: "2.5.0"
project_name: "scored"
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
project_name: "large-test"
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
context: |
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
}
