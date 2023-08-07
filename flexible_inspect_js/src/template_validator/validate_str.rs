use crate::error::WasmValidationErrorIterator;

use super::*;
use std::sync::Arc;

#[wasm_bindgen(js_name = "TemplateValidator")]
pub struct WasmTemplateValidator(TemplateValidator<Vec<Cartridge<Rule>>, Arc<str>>);

#[wasm_bindgen(js_class = "TemplateValidator")]
impl WasmTemplateValidator {
    #[wasm_bindgen(constructor)]
    pub fn new(cartridges: JsValue) -> Result<WasmTemplateValidator, JsValue> {
        console_error_panic_hook::set_once();
        Ok(WasmTemplateValidator(TemplateValidator::new(serde_wasm_bindgen::from_value(cartridges)
        .map_err(|_| {
            JsValue::from_str(" (TemplateValidator) Cartridge` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `Cartridge` ( [ Cartridge, Cartridge, Cartridge ] ) type for the `TemplateValidator`")
        })?)))
    }

    pub fn validate(&self, data: String) -> WasmValidationErrorIterator {
        if let Err(value) = self.0.validate(data.into()) {
            return WasmValidationErrorIterator::new(
                value.into_iter().collect(),
            )
        }
        WasmValidationErrorIterator::new(vec![])
    }

    pub async fn async_validate(&self, data: String) -> WasmValidationErrorIterator {
        if let Err(value) = self.0.async_validate(data.into()).await {
            return WasmValidationErrorIterator::new(value.into_iter().collect());
        }
        WasmValidationErrorIterator::new(vec![])
    }
}
