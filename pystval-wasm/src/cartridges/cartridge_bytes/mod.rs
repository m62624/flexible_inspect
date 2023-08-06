mod modifiers;
use super::*;

#[wasm_bindgen(js_name = "CartridgeBytes")]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WasmCartridgeBytes(Cartridge<RuleBytes>);

#[wasm_bindgen(js_class = "CartridgeBytes")]
impl WasmCartridgeBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(
        error_code: i32,
        message: String,
        root_rules: JsValue,
    ) -> Result<WasmCartridgeBytes, JsError> {
        console_error_panic_hook::set_once();
        Ok(Self(
            Cartridge::new(error_code, message, serde_wasm_bindgen::from_value::<Vec<RuleBytes>>(root_rules)
        .map_err(|_| JsError::new("A collection of type `RuleBytes` is expected [ RuleBytes, RuleBytes ,RuleBytes ]"))?
        .into_iter()
        .map(|rule| rule.into())))
    )
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<WasmCartridgeBytes> for Cartridge<RuleBytes> {
    fn from(value: WasmCartridgeBytes) -> Self {
        value.0
    }
}
