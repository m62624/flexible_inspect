use super::*;
use crate::error::WasmValidationErrorIterator;
use std::sync::Arc;

#[wasm_bindgen(js_name = "TemplateValidatorBytes")]
pub struct WasmTemplateValidatorBytes(TemplateValidator<Vec<Cartridge<RuleBytes>>, Arc<[u8]>>);

#[wasm_bindgen(js_class = "TemplateValidatorBytes")]
impl WasmTemplateValidatorBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(cartridges: JsValue) -> Result<WasmTemplateValidatorBytes, JsValue> {
        console_error_panic_hook::set_once();
        Ok(WasmTemplateValidatorBytes(TemplateValidator::new(serde_wasm_bindgen::from_value(cartridges)
        .map_err(|_| {
            JsValue::from_str(" (TemplateValidatorBytes) CartridgeBytes` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `CartridgeBytes` ( [ CartridgeBytes, CartridgeBytes, CartridgeBytes ] ) type for the `TemplateValidatorBytes`")
        })?)))
    }

    pub fn validate(&self, data: &[u8]) -> Option<WasmValidationErrorIterator> {
        if let Err(value) = self.0.validate(data.into()) {
            return Some(WasmValidationErrorIterator::new(
                value.into_iter().collect(),
            ));
        }
        None
    }

    pub async fn async_validate(&self, data: &[u8]) -> Option<WasmValidationErrorIterator> {
        if let Err(value) = self.0.async_validate(data.into()).await {
            return Some(WasmValidationErrorIterator::new(
                value.into_iter().collect(),
            ));
        }
        None
    }
}
