use super::*;
use crate::error::WasmValidationErrorIterator;
use std::sync::Arc;

/// The structure for creating unique validators, load different `cartridges` to validate data.

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
        Ok(WasmTemplateValidator(TemplateValidator::new(
            serde_wasm_bindgen::from_value(cartridges)
                .map_err(|_| JsValue::from_str(ERR_OPTION_CARTRIDGE))?,
        )))
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
