//! WASM bindings for AgenticCognition

use wasm_bindgen::prelude::*;
use agentic_cognition::{CognitionStore, WriteEngine, QueryEngine};

#[wasm_bindgen]
pub fn version() -> String {
    "0.1.0".to_string()
}

#[wasm_bindgen]
pub fn create_model() -> Result<String, JsValue> {
    let store = CognitionStore::new();
    let engine = WriteEngine::new(store);
    let id = engine.create_model()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(id.to_string())
}
