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
/// Pseudo iterator that stores received errors from validations
#[wasm_bindgen(js_name = "ValidationErrorIterator")]
pub struct WasmValidationErrorIterator(pub(crate) Vec<ValidationError>);

#[wasm_bindgen(js_class = "ValidationErrorIterator")]
impl WasmValidationErrorIterator {
    /// Returns the `next` error from the iterator or `None` if there are no more errors.
    /// # Example:
    /// ```js
    /// while (errors.hasNext()) {
    ///    const error = errors.next();
    ///   console.log(error.code, error.message);
    /// }
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<WasmBaseValidationError> {
        self.0.pop().map(|error| WasmBaseValidationError {
            code: error.get_code(),
            message: error.get_message().to_owned(),
        })
    }

    /// Returns `true` if the iterator has not returned all errors yet.
    #[wasm_bindgen(js_name = "hasNext")]
    pub fn has_next(&self) -> bool {
        !self.0.is_empty()
    }

    /// Used instead of `hasNext()` & `next()`, execute the callback as many times as the number of errors in the pseudo iterator
    /// # Arguments:
    /// * `callback` - accepts callback without parameters
    pub fn for_each_0(&self, callback: js_sys::Function) -> Result<Vec<JsValue>, JsValue> {
        self.0.iter().try_fold(Vec::new(), |mut acc, _| {
            acc.push(callback.call0(&callback)?);
            Ok(acc)
        })
    }

    /// Used instead of `hasNext()` & `next()`, execute the callback as many times as the number of errors in the pseudo iterator
    /// # Arguments:
    /// * `callback` - accepts callback with two parameters. Uses reserved parameters,
    /// * the first is the `error code`
    /// * the second is the `error message`
    pub fn for_each_1(&self, callback: js_sys::Function) -> Result<Vec<JsValue>, JsValue> {
        self.0.iter().try_fold(Vec::new(), |mut acc, item| {
            acc.push(callback.call2(
                &callback,
                &JsValue::from(item.get_code()),
                &JsValue::from(item.get_message()),
            )?);
            Ok(acc)
        })
    }

    /// Used instead of `hasNext()` & `next()`, execute the callback as many times as the number of errors in the pseudo iterator
    /// Uses reserved parameters,
    /// * first - `error code`, 
    /// * second - `error message`\
    /// and any other parameters in the `collection`
    pub fn for_each_2(
        &self,
        callback: js_sys::Function,
        params: js_sys::Array,
    ) -> Result<Vec<JsValue>, JsValue> {
        self.0.iter().try_fold(Vec::new(), |mut acc, item| {
            acc.push(callback.call3(
                &callback,
                &JsValue::from(item.get_code()),
                &JsValue::from(item.get_message()),
                &params,
            )?);
            Ok(acc)
        })
    }

    /// `len()` returns the number of errors in the pseudo iterator
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// `is_empty()` returns `true` if the pseudo iterator has no errors
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl WasmValidationErrorIterator {
    pub fn new(collection: Vec<ValidationError>) -> Self {
        Self(collection)
    }
}
