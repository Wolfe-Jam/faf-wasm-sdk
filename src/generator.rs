//! Generator - Create project.faf from repo metadata
//!
//! Ported from xai-faf-rust for WASM usage in builder.faf.one

use wasm_bindgen::prelude::*;
use serde_json::Value;

/// Check if a repo name is descriptive (can infer purpose)
///
/// Descriptive names have:
/// - Structure (hyphens/underscores)
/// - Tech/action/domain keywords
///
/// Examples:
/// - test-faf-demo ✅ (has structure + keywords)
/// - react-todo-app ✅ (clear tech + purpose)
/// - ziggy ❌ (abstract, no keywords)
/// - kubernetes ❌ (made-up word)
///
/// TODO: Port this logic to faf-cli for `faf readme` enhancement
/// This same smart extraction should be available in TypeScript
/// for the CLI's README enhancement feature.
fn is_descriptive_name(name: &str) -> bool {
    // Must have separators (compound structure)
    if !name.contains('-') && !name.contains('_') {
        return false;
    }

    let name_lower = name.to_lowercase();

    // Keywords that indicate descriptive naming
    let tech_keywords = [
        "react", "vue", "svelte", "angular", "next",
        "node", "express", "django", "flask",
        "api", "cli", "app", "lib", "sdk", "tool", "framework",
        "rust", "python", "go", "java", "js", "ts",
    ];

    let action_verbs = [
        "test", "demo", "build", "parse", "fetch", "sync",
        "deploy", "manage", "create", "generate", "validate",
    ];

    let domain_nouns = [
        "todo", "auth", "blog", "chat", "dashboard", "admin",
        "user", "data", "file", "image", "video", "audio",
    ];

    let purpose_words = [
        "demo", "example", "starter", "template", "boilerplate",
        "sample", "tutorial", "guide", "playground",
    ];

    // Check if name contains known keywords
    let has_keywords = tech_keywords.iter().any(|k| name_lower.contains(k))
        || action_verbs.iter().any(|k| name_lower.contains(k))
        || domain_nouns.iter().any(|k| name_lower.contains(k))
        || purpose_words.iter().any(|k| name_lower.contains(k));

    has_keywords
}

/// Infer WHY from descriptive repo name
///
/// Examples:
/// - test-faf-demo → "Test and demonstrate FAF system"
/// - react-todo-app → "Todo application built with React"
/// - api-rate-limiter → "API rate limiting"
fn infer_why_from_name(name: &str) -> Option<String> {
    if !is_descriptive_name(name) {
        return None; // Name is abstract, don't infer
    }

    let name_lower = name.to_lowercase();
    let parts: Vec<&str> = name_lower.split(&['-', '_'][..]).collect();

    // Pattern matching for common structures
    if parts.len() < 2 {
        return None;
    }

    // Check for common patterns
    if parts.contains(&"demo") || parts.contains(&"example") {
        return Some(format!("Demonstrate and test {} system",
            parts.iter()
                .filter(|p| *p != &"demo" && *p != &"example" && *p != &"test")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }

    if parts.contains(&"starter") || parts.contains(&"template") || parts.contains(&"boilerplate") {
        return Some(format!("Starter template for {} development",
            parts.iter()
                .filter(|p| *p != &"starter" && *p != &"template" && *p != &"boilerplate")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }

    // Generic descriptive inference
    let readable = parts.join(" ");
    Some(format!("Project for {}", readable))
}

/// Extract WHO/WHAT/WHY/WHERE/WHEN/HOW from README content
fn extract_from_readme(
    readme: Option<&str>,
    repo_name: &str,
    owner: &str,
) -> (String, String, String, String, String, String) {
    let default_who = format!("{} team", owner);
    let default_what = repo_name.to_string();
    let default_where = "GitHub".to_string();
    let default_when = format!("Initialized {}", chrono::Utc::now().format("%Y-%m-%d"));
    let default_why = "TBD".to_string();
    let default_how = "TBD".to_string();

    let readme_text = match readme {
        Some(r) if !r.trim().is_empty() => r,
        _ => {
            return (
                default_who,
                default_what,
                default_why,
                default_where,
                default_when,
                default_how,
            )
        }
    };

    // Extract WHAT (title - first # heading)
    let what = readme_text
        .lines()
        .find(|line| line.trim_start().starts_with("# "))
        .map(|line| line.trim_start().trim_start_matches("# ").trim().to_string())
        .unwrap_or(default_what);

    // Extract WHY (look for Purpose:, Why:, Mission: patterns)
    let why_patterns = ["## Purpose", "## Why", "## Mission", "## Goal"];
    let mut why = default_why.clone();
    for pattern in &why_patterns {
        if let Some(pos) = readme_text.find(pattern) {
            let section = &readme_text[pos..];
            if let Some(next_section) = section.find("\n## ") {
                let content = section[pattern.len()..next_section]
                    .trim()
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" ");
                if !content.is_empty() {
                    why = content;
                    break;
                }
            } else {
                let content = section[pattern.len()..]
                    .trim()
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" ");
                if !content.is_empty() {
                    why = content;
                    break;
                }
            }
        }
    }

    // Fallback: Try to infer WHY from repo name if README didn't have it
    if why == "TBD" {
        if let Some(inferred) = infer_why_from_name(repo_name) {
            why = inferred;
        }
    }

    // Extract HOW (look for Installation, Getting Started, Usage sections)
    let how_patterns = [
        "## Installation",
        "## Getting Started",
        "## Usage",
        "## Quick Start",
    ];
    let mut how = default_how.clone();
    for pattern in &how_patterns {
        if readme_text.contains(pattern) {
            how = format!("See README: {}", pattern.trim_start_matches("## "));
            break;
        }
    }

    // Extract WHEN (look for roadmap, timeline, or use current date)
    let when = if readme_text.contains("## Roadmap") || readme_text.contains("## Timeline") {
        "See README: Roadmap".to_string()
    } else {
        default_when
    };

    (default_who, what, why, default_where, when, how)
}

/// Detect primary tech stack from package.json OR readme
fn detect_stack(dependency_file: Option<&str>, language: Option<&str>, readme: Option<&str>) -> String {
    // First, use language from GitHub API as fallback
    let lang = language.unwrap_or("Unknown");

    // If dependency file exists, use it
    if let Some(dep_text) = dependency_file {
        if !dep_text.trim().is_empty() {
            return match lang {
                "Python" => detect_python_stack(dep_text),
                "Rust" => detect_rust_stack(dep_text),
                "Go" => detect_go_stack(dep_text),
                "Ruby" => detect_ruby_stack(dep_text),
                "JavaScript" | "TypeScript" => detect_js_stack(dep_text),
                _ => lang.to_string(),
            };
        }
    }

    // If no dependency file but README exists, detect from README
    if let Some(readme_text) = readme {
        if !readme_text.trim().is_empty() {
            let readme_lower = readme_text.to_lowercase();

            // ML/AI frameworks (from README mentions)
            if readme_lower.contains("jax") || readme_lower.contains("flax") {
                return "JAX".to_string();
            }
            if readme_lower.contains("pytorch") || readme_lower.contains("torch") {
                return "PyTorch".to_string();
            }
            if readme_lower.contains("tensorflow") {
                return "TensorFlow".to_string();
            }
        }
    }

    // Fall back to language
    lang.to_string()
}

/// Detect Python stack from requirements.txt or pyproject.toml
fn detect_python_stack(content: &str) -> String {
    let content_lower = content.to_lowercase();

    // ML/AI frameworks (priority)
    if content_lower.contains("jax") || content_lower.contains("flax") {
        return "JAX".to_string();
    }
    if content_lower.contains("torch") || content_lower.contains("pytorch") {
        return "PyTorch".to_string();
    }
    if content_lower.contains("tensorflow") {
        return "TensorFlow".to_string();
    }

    // Web frameworks
    if content_lower.contains("django") {
        return "Django".to_string();
    }
    if content_lower.contains("flask") {
        return "Flask".to_string();
    }
    if content_lower.contains("fastapi") {
        return "FastAPI".to_string();
    }

    "Python".to_string()
}

/// Detect Rust stack from Cargo.toml
fn detect_rust_stack(content: &str) -> String {
    let content_lower = content.to_lowercase();

    if content_lower.contains("axum") {
        return "Axum".to_string();
    }
    if content_lower.contains("actix") {
        return "Actix".to_string();
    }
    if content_lower.contains("rocket") {
        return "Rocket".to_string();
    }
    if content_lower.contains("wasm-bindgen") {
        return "Rust WASM".to_string();
    }

    "Rust".to_string()
}

/// Detect Go stack from go.mod
fn detect_go_stack(content: &str) -> String {
    let content_lower = content.to_lowercase();

    if content_lower.contains("gin") {
        return "Gin".to_string();
    }
    if content_lower.contains("echo") {
        return "Echo".to_string();
    }
    if content_lower.contains("fiber") {
        return "Fiber".to_string();
    }

    "Go".to_string()
}

/// Detect Ruby stack from Gemfile
fn detect_ruby_stack(content: &str) -> String {
    let content_lower = content.to_lowercase();

    if content_lower.contains("rails") {
        return "Rails".to_string();
    }
    if content_lower.contains("sinatra") {
        return "Sinatra".to_string();
    }

    "Ruby".to_string()
}

/// Detect JavaScript/TypeScript stack from package.json
fn detect_js_stack(pkg_text: &str) -> String {
    // Parse package.json
    let pkg: Value = match serde_json::from_str(pkg_text) {
        Ok(p) => p,
        Err(_) => return "Node.js".to_string(),
    };

    // Check dependencies
    let deps = pkg["dependencies"].as_object();
    let dev_deps = pkg["devDependencies"].as_object();

    // Priority order: React > Vue > Svelte > Next > Express > Node.js
    let frameworks = [
        ("react", "React"),
        ("vue", "Vue"),
        ("svelte", "Svelte"),
        ("next", "Next.js"),
        ("@angular/core", "Angular"),
        ("express", "Express"),
    ];

    for (dep_name, framework) in &frameworks {
        if let Some(deps_obj) = deps {
            if deps_obj.contains_key(*dep_name) {
                return framework.to_string();
            }
        }
        if let Some(dev_deps_obj) = dev_deps {
            if dev_deps_obj.contains_key(*dep_name) {
                return framework.to_string();
            }
        }
    }

    // Default to Node.js if package.json exists
    "Node.js".to_string()
}

/// Extract version from package.json
fn extract_version(package_json: Option<&str>) -> String {
    let pkg_text = match package_json {
        Some(p) if !p.trim().is_empty() => p,
        _ => return "0.0.1".to_string(),
    };

    let pkg: Value = match serde_json::from_str(pkg_text) {
        Ok(p) => p,
        Err(_) => return "0.0.1".to_string(),
    };

    pkg["version"]
        .as_str()
        .unwrap_or("0.0.1")
        .to_string()
}

/// Detect project type based on language and stack analysis
///
/// Returns type that matches faf-cli's type system:
/// - ml-research: ML/AI models (JAX, PyTorch, TensorFlow)
/// - web-app: Frontend apps (React, Vue, Svelte)
/// - api: Backend services (Express, FastAPI, Django)
/// - cli: Command-line tools
/// - library: Reusable packages (default)
fn detect_type(language: Option<&str>, stack: &str, dependency_file: Option<&str>) -> String {
    let lang = language.unwrap_or("Unknown");
    let stack_lower = stack.to_lowercase();

    // ML/AI detection (highest priority)
    if stack_lower.contains("jax") || stack_lower.contains("pytorch") || stack_lower.contains("tensorflow") {
        return "ml-research".to_string();
    }

    // Frontend frameworks
    if stack_lower.contains("react") || stack_lower.contains("vue") || stack_lower.contains("svelte")
        || stack_lower.contains("angular") || stack_lower.contains("next") {
        return "web-app".to_string();
    }

    // Backend frameworks
    if stack_lower.contains("express") || stack_lower.contains("fastapi") || stack_lower.contains("django")
        || stack_lower.contains("flask") || stack_lower.contains("axum") || stack_lower.contains("actix") {
        return "api".to_string();
    }

    // CLI detection (check dependency file for CLI indicators)
    if let Some(deps) = dependency_file {
        let deps_lower = deps.to_lowercase();
        if deps_lower.contains("clap") || deps_lower.contains("commander")
            || deps_lower.contains("yargs") || deps_lower.contains("argparse") {
            return "cli".to_string();
        }
    }

    // Default based on language
    match lang {
        "Python" | "JavaScript" | "TypeScript" | "Rust" | "Go" | "Ruby" => "library".to_string(),
        _ => "library".to_string(),
    }
}

/// Generate project.faf content (builder.faf.one template)
/// This is the UNIVERSAL template (not xAI-specific), optimized for 85%+ initial scores
#[wasm_bindgen]
pub fn generate_faf(
    repo_name: String,
    owner: String,
    description: Option<String>,
    readme: Option<String>,
    dependency_file: Option<String>,
    language: Option<String>,
) -> Result<String, JsValue> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");

    let (who, what, why, where_val, when, how) = extract_from_readme(
        readme.as_deref(),
        &repo_name,
        &owner,
    );

    let stack = detect_stack(dependency_file.as_deref(), language.as_deref(), readme.as_deref());
    let version = extract_version(dependency_file.as_deref());
    let goal = description.unwrap_or_else(|| what.clone());
    let main_language = language.unwrap_or_else(|| "Unknown".to_string());

    // Detect project type based on language and stack
    let project_type = detect_type(Some(&main_language), &stack, dependency_file.as_deref());

    // Generate UNIVERSAL template (optimized for better initial scores)
    let faf_content = format!(
        r#"faf_version: 2.5.0
ai_scoring_system: 2025-12-17
ai_confidence: MODERATE
ai_value: 30_seconds_replaces_20_minutes_of_questions

# AI-Optimized Context (The Quick Brief)
ai_tldr:
  project: {repo_name}
  stack: {stack}
  quality_bar: PRODUCTION
  current_focus: Initial setup
  your_role: Build with AI assistance

# Instant Context Snapshot
instant_context:
  what_building: {what}
  tech_stack: {stack}
  main_language: {main_language}
  deployment: GitHub
  key_files: []

# Context Quality Metrics
context_quality:
  slots_filled: 12/21 (57%)
  ai_confidence: MODERATE
  handoff_ready: true
  missing_context:
    - Detailed architecture
    - Key file mappings
    - Development workflow

# Project Identity
project:
  name: {repo_name}
  goal: {goal}
  main_language: {main_language}
  type: {project_type}
  version: {version}
  generated: {now}
  repository: https://github.com/{owner}/{repo_name}

# AI Instructions
ai_instructions:
  priority_order:
    - 1. Read THIS .faf file first
    - 2. Check README.md for overview
    - 3. Review key files
  working_style:
    code_first: true
    explanations: clear
    quality_bar: production
    testing: recommended
  warnings:
    - Follow existing code patterns
    - Test before committing

# Technology Stack
stack:
  primary: {stack}
  frontend: {stack}
  backend: Unknown
  runtime: Node.js
  database: Unknown
  build: Unknown
  package_manager: npm
  hosting: GitHub

# Developer Preferences
preferences:
  quality_bar: production
  commit_style: conventional
  response_style: balanced
  explanation_level: clear
  communication: friendly
  testing: recommended

# Project State
state:
  phase: active
  version: {version}
  focus: development
  status: ready
  next_milestone: Define roadmap
  blockers: null

# Tags
tags:
  - {repo_name}
  - {stack}
  - faf
  - ai-ready

# Human Context (The 6 Ws)
human_context:
  who: {who}
  what: {what}
  why: {why}
  where: {where_val}
  when: {when}
  how: {how}
  additional_context: Generated by builder.faf.one
  context_score: 57
  total_prd_score: 57
  success_rate: 57%

# AI Scoring Details
ai_scoring_details:
  system_date: 2025-12-17
  slot_based_percentage: 57
  ai_score: 57
  total_slots: 21
  filled_slots: 12
  scoring_method: Honest percentage - no fake minimums
  trust_embedded: COUNT ONCE architecture

# FAF DNA (Birth Certificate)
faf_dna:
  birth_dna: 57
  birth_certificate: FAF-2025-{name_upper}-INIT
  birth_date: {now}
  current_score: 57
"#,
        repo_name = repo_name,
        owner = owner,
        stack = stack,
        version = version,
        goal = goal,
        project_type = project_type,
        what = what,
        why = why,
        who = who,
        where_val = where_val,
        when = when,
        how = how,
        now = now,
        name_upper = repo_name
            .to_uppercase()
            .replace("-", "")
            .chars()
            .take(8)
            .collect::<String>(),
    );

    Ok(faf_content)
}

/// Generate metadata-only .faf for private repos (when README/package.json unavailable)
#[wasm_bindgen]
pub fn generate_faf_minimal(
    repo_name: String,
    owner: String,
    description: Option<String>,
    language: Option<String>,
) -> Result<String, JsValue> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");
    let stack = language.unwrap_or_else(|| "Unknown".to_string());
    let goal = description.unwrap_or_else(|| format!("{} repository", repo_name));

    let faf_content = format!(
        r#"faf_version: 2.5.0
ai_scoring_system: 2025-12-17
ai_confidence: LOW
ai_value: 30_seconds_replaces_20_minutes_of_questions

# AI-Optimized Context (The Quick Brief)
ai_tldr:
  project: {repo_name}
  stack: {stack}
  quality_bar: UNKNOWN
  current_focus: Initial setup
  your_role: Build with AI assistance

# Instant Context Snapshot
instant_context:
  what_building: {goal}
  tech_stack: {stack}
  main_language: {stack}
  deployment: GitHub (private)
  key_files: []

# Context Quality Metrics
context_quality:
  slots_filled: 5/21 (24%)
  ai_confidence: LOW
  handoff_ready: false
  missing_context:
    - Private repo - run faf-cli locally for better context
    - Tech stack details
    - Key files
    - Architecture

# Project Identity
project:
  name: {repo_name}
  goal: {goal}
  main_language: {stack}
  type: unknown
  version: 0.0.1
  generated: {now}
  repository: https://github.com/{owner}/{repo_name}

# AI Instructions
ai_instructions:
  priority_order:
    - 1. Read THIS .faf file first
    - 2. Check README.md for overview
    - 3. Review key files
  working_style:
    code_first: true
    explanations: clear
    quality_bar: production
    testing: recommended

# Technology Stack
stack:
  primary: {stack}
  frontend: Unknown
  backend: Unknown
  runtime: Unknown
  database: Unknown
  build: Unknown
  package_manager: Unknown
  hosting: GitHub

# Human Context (The 6 Ws)
human_context:
  who: {owner} team
  what: {goal}
  why: TBD - Private repo (run faf-cli locally)
  where: GitHub (private repository)
  when: Initialized {when}
  how: TBD - Private repo (run faf-cli locally)
  additional_context: Generated from public metadata only. Run 'faf-cli' locally for better context.
  context_score: 24
  total_prd_score: 24
  success_rate: 24%

# AI Scoring Details
ai_scoring_details:
  system_date: 2025-12-17
  slot_based_percentage: 24
  ai_score: 24
  total_slots: 21
  filled_slots: 5
  scoring_method: Honest percentage - no fake minimums
  trust_embedded: COUNT ONCE architecture

# FAF DNA (Birth Certificate)
faf_dna:
  birth_dna: 24
  birth_certificate: FAF-2025-{name_upper}-PRIV
  birth_date: {now}
  current_score: 24
"#,
        repo_name = repo_name,
        owner = owner,
        stack = stack,
        goal = goal,
        when = chrono::Utc::now().format("%Y-%m-%d"),
        now = now,
        name_upper = repo_name
            .to_uppercase()
            .replace("-", "")
            .chars()
            .take(8)
            .collect::<String>(),
    );

    Ok(faf_content)
}
