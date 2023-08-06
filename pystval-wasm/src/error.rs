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

    pub fn for_each(&self, callback: js_sys::Function) -> Result<Vec<JsValue>, JsValue> {
        self.0.iter().try_fold(Vec::new(), |mut acc, item| {
            let result = callback.call2(
                &callback,
                &JsValue::from(item.get_code()),
                &JsValue::from(item.get_message()),
            )?;
            acc.push(result);
            Ok(acc)
        })
    }

    pub fn if_error(&self, callback: js_sys::Function) -> Result<Vec<JsValue>, JsValue> {
        if self.0.is_empty() {
            Ok(Vec::new())
        } else {
            self.for_each(callback)
        }
    }

    pub fn if_ok(&self, callback: js_sys::Function) -> Result<JsValue, JsValue> {
        if self.0.is_empty() {
            Ok(callback.call0(&callback)?)
        } else {
            Ok(JsValue::NULL)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl WasmValidationErrorIterator {
    pub fn new(collection: Vec<Box<dyn ValidationError + Send>>) -> Self {
        Self(collection)
    }
}
