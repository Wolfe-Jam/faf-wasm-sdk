//! WASM-safe error handling

use wasm_bindgen::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FafError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Scoring error: {0}")]
    Scoring(String),
}

impl From<FafError> for JsValue {
    fn from(err: FafError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}
