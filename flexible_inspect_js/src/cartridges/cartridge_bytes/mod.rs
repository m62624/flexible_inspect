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
    ) -> Result<WasmCartridgeBytes, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self(
            Cartridge::new(error_code, message, serde_wasm_bindgen::from_value::<Vec<RuleBytes>>(root_rules)
        .map_err(|_| JsValue::from_str(" (CartridgeBytes) RuleBytes` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `RuleBytes` ( [ RuleBytes, RuleBytes, RuleBytes ] ) type for the `CartridgeBytes`"))?
        .into_iter()
        ))
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
