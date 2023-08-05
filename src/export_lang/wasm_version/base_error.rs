use crate::core::base_error::PystvalError;
use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = PystvalError)]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmPystvalError(PystvalError);

impl WasmPystvalError {
    pub fn new(id: i32, msg: String) -> Self {
        Self(PystvalError::new(id, msg))
    }
}

#[wasm_bindgen(js_class = PystvalError)]
impl WasmPystvalError {
    pub fn get_code(&self) -> i32 {
        self.0.id
    }

    pub fn get_message(&self) -> String {
        self.0.msg.to_string()
    }
}

impl fmt::Display for WasmPystvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
