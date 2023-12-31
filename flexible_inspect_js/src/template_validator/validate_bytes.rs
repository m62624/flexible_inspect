use super::*;
use crate::error::WasmValidationErrorIterator;
use std::sync::Arc;

/// The structure for creating unique validators, load different `cartridges` to validate data
/// (recommendation to use the string version if possible. More information on `Rule`.)
#[wasm_bindgen(js_name = "TemplateValidatorBytes")]
pub struct WasmTemplateValidatorBytes(TemplateValidator<Vec<Cartridge<RuleBytes>>, Arc<[u8]>>);

#[wasm_bindgen(js_class = "TemplateValidatorBytes")]
impl WasmTemplateValidatorBytes {
    #[wasm_bindgen(constructor)]
    /// Constructor for creating a validator
    /// # Arguments:
    /// * `cartridges` - \[ `CartridgeBytes`, `CartridgeBytes`, `CartridgeBytes` \] (collection)
    pub fn new(cartridges: JsValue) -> Result<WasmTemplateValidatorBytes, JsValue> {
        console_error_panic_hook::set_once();
        Ok(WasmTemplateValidatorBytes(TemplateValidator::new(
            serde_wasm_bindgen::from_value(cartridges)
                .map_err(|_| JsValue::from_str(ERR_OPTION_CARTRIDGE_BYTES))?,
        )))
    }

    /// Method for validating data
    ///
    /// **Notes:**
    /// * If possible, differentiate cartridges clearly by purpose,
    /// without making a common validator for different purposes
    ///
    pub fn validate(&self, data: &[u8]) -> Option<WasmValidationErrorIterator> {
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
    pub async fn async_validate(&self, data: &[u8]) -> Option<WasmValidationErrorIterator> {
        if let Err(value) = self.0.async_validate(data.into()).await {
            return Some(WasmValidationErrorIterator::new(
                value.into_iter().collect(),
            ));
        }
        None
    }
}
