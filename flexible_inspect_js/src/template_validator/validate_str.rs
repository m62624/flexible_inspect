use crate::error::WasmValidationErrorIterator;

use super::*;
use std::sync::Arc;

/// Constructor for creating a validator
/// Load different `Cartridge` into the `construcrtor` to create a validator for different situations
#[wasm_bindgen(js_name = "TemplateValidator")]
pub struct WasmTemplateValidator(TemplateValidator<Vec<Cartridge<Rule>>, Arc<str>>);

#[wasm_bindgen(js_class = "TemplateValidator")]
impl WasmTemplateValidator {
    /// Constructor for creating a validator
    /// # Arguments:
    /// * `cartridges` - \[ `Cartridge`, `Cartridge`, `Cartridge` \] (collection)
    #[wasm_bindgen(constructor)]
    pub fn new(cartridges: JsValue) -> Result<WasmTemplateValidator, JsValue> {
        console_error_panic_hook::set_once();
        Ok(WasmTemplateValidator(TemplateValidator::new(serde_wasm_bindgen::from_value(cartridges)
        .map_err(|_| {
            JsValue::from_str(" (TemplateValidator) Cartridge` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `Cartridge` ( [ Cartridge, Cartridge, Cartridge ] ) type for the `TemplateValidator`")
        })?)))
    }

    /// Method for validating data
    ///
    /// **Notes:**
    /// * If possible, differentiate cartridges clearly by purpose,
    /// without making a common validator for different purposes
    ///
    pub fn validate(&self, data: String) -> Option<WasmValidationErrorIterator> {
        if let Err(value) = self.0.validate(data.into()) {
            return Some(WasmValidationErrorIterator::new(
                value.into_iter().collect(),
            ));
        }
        None
    }

    /// Method for validating data. Runs validation of each cartridge asynchronously
    ///
    /// **Notes:**
    /// * If possible, differentiate cartridges clearly by purpose,
    /// without making a common validator for different purposes
    ///
    pub async fn async_validate(&self, data: String) -> Option<WasmValidationErrorIterator> {
        if let Err(value) = self.0.async_validate(data.into()).await {
            return Some(WasmValidationErrorIterator::new(
                value.into_iter().collect(),
            ));
        }
        None
    }
}
