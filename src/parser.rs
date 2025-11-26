//! FAF YAML Parser - serde_yaml for WASM
//!
//! Handles .faf file parsing with proper error messages.

use wasm_bindgen::prelude::*;
use crate::types::FafDna;

/// Parse FAF YAML content
pub fn parse_faf(content: &str) -> Result<FafDna, String> {
    serde_yaml::from_str(content)
        .map_err(|e| format!("YAML parse error: {}", e))
}

/// Parse FAF YAML with detailed error location
pub fn parse_faf_detailed(content: &str) -> Result<FafDna, ParseError> {
    serde_yaml::from_str(content).map_err(|e| ParseError {
        message: e.to_string(),
        line: e.location().map(|l| l.line()),
        column: e.location().map(|l| l.column()),
    })
}

/// Parse error with location info
#[wasm_bindgen]
pub struct ParseError {
    message: String,
    line: Option<usize>,
    column: Option<usize>,
}

#[wasm_bindgen]
impl ParseError {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn line(&self) -> Option<usize> {
        self.line
    }

    #[wasm_bindgen(getter)]
    pub fn column(&self) -> Option<usize> {
        self.column
    }
}
