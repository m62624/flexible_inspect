use crate::{
    core::{cartridges::CartridgeBase, message::filling_message, rules::next::NextStep},
    export_lang::wasm_version::base_error::WasmPystvalError,
};

use super::*;

#[wasm_bindgen(js_name = "TemplateValidator")]
#[derive(Debug)]
pub struct WasmTemplateValidator(TemplateValidator<Vec<Cartridge<Rule>>, Arc<str>>);

#[wasm_bindgen(js_class = "TemplateValidator")]
impl WasmTemplateValidator {
    #[wasm_bindgen(constructor)]
    pub fn new(cartridges: Vec<JsValue>) -> Result<WasmTemplateValidator, JsValue> {
        console_error_panic_hook::set_once();
        if cartridges.is_empty() {
            panic!("Cartridges must not be empty");
        }
        Ok(Self(TemplateValidator::new(_to_rust_for_extend(cartridges, format!("\n` (TemplateValidator) Cartridge` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `Cartridge` (collection format) type for the `TemplateValidator`").as_str(), "\nYou must specify at least one cartridge for the `TemplateValidator` (collection format) [ Cartridge, Cartridge, Cartridge ]")?)))
    }
}

#[async_trait]
impl WasmValidatorBase<String> for WasmTemplateValidator {
    fn validate(&self, data: String) -> Option<Vec<JsValue>> {
        let mut error = Vec::new();
        for cartridge in self.0.cartridges.iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data.as_str()) {
                error.push(
                    serde_wasm_bindgen::to_value(&WasmPystvalError::new(
                        <Cartridge<Rule> as CartridgeBase<Rule, &str>>::get_id(cartridge),
                        filling_message(
                            <Cartridge<Rule> as CartridgeBase<Rule, &str>>::get_message(cartridge),
                            extra_with_value,
                        ),
                    ))
                    .unwrap(),
                );
            }
        }
        if error.is_empty() {
            None
        } else {
            Some(error)
        }
    }

    async fn async_validate(&self, data: String) -> Result<JsValue, JsValue> {
        // self.validate(data).await
        let result = wasm_bindgen_futures::future_to_promise(self.validate(data).await);
        todo!()
    }
}
