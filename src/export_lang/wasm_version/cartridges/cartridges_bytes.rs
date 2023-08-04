

use super::*;

#[wasm_bindgen(js_name = CartridgeBytes)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct WasmCartridgeBytes(pub(crate) Cartridge<RuleBytes>);

#[wasm_bindgen(js_class = CartridgeBytes)]
impl WasmCartridgeBytes {
    pub fn start_build(
        id: i64,
        message: String,
        root_rules: Vec<JsValue>,
    ) -> Result<WasmCartridgeBytes, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self(Cartridge::new(
            id,
            message,
            WasmRuleBytes::_to_rust_for_extend(root_rules, "RuleBytes")?,
        )))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}