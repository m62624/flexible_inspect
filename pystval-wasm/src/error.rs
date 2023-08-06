use super::*;

#[wasm_bindgen(js_name = "ValidationError")]
#[derive(Debug)]
pub struct WasmBaseValidationError {
    code: i32,
    message: String,
}

#[wasm_bindgen(js_class = "ValidationError")]
impl WasmBaseValidationError {
    pub fn get_code(&self) -> i32 {
        self.code
    }

    pub fn get_message(&self) -> String {
        self.message.to_owned()
    }
}

#[wasm_bindgen(js_name = "ValidationErrorIterator")]
pub struct WasmValidationErrorIterator(Vec<Box<dyn ValidationError + Send>>);

#[wasm_bindgen(js_class = "ValidationErrorIterator")]
impl WasmValidationErrorIterator {
    pub fn next(&mut self) -> Option<WasmBaseValidationError> {
        self.0.pop().map(|error| WasmBaseValidationError {
            code: error.get_code(),
            message: error.get_message().to_owned(),
        })
    }
}


impl WasmValidationErrorIterator{
    pub fn new(collection: Vec<Box<dyn ValidationError + Send>>) -> Self {
        Self(collection)
    }
}