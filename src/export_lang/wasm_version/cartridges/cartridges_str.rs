
use super::*;

#[wasm_bindgen(js_name = Cartridge)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct WasmCartridge(pub(crate) Cartridge<Rule>);

#[wasm_bindgen(js_class = Cartridge)]
impl WasmCartridge {
    pub fn start_build(
        id: i64,
        message: String,
        root_rules: Vec<JsValue>,
    ) -> Result<WasmCartridge, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self(Cartridge::new(
            id,
            message,
            WasmRule::_to_rust_for_extend(root_rules, "Rule")?,
        )))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}
