mod modifiers;
use super::*;

#[wasm_bindgen(js_name = "Cartridge")]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WasmCartridge(Cartridge<Rule>);

#[wasm_bindgen(js_class = "Cartridge")]
impl WasmCartridge {
    #[wasm_bindgen(constructor)]
    pub fn new(
        error_code: i32,
        message: String,
        root_rules: JsValue,
    ) -> Result<WasmCartridge, JsError> {
        console_error_panic_hook::set_once();
        Ok(Self(Cartridge::new(
            error_code,
            message,
            serde_wasm_bindgen::from_value::<Vec<Rule>>(root_rules)
                .map_err(|_| {
                    JsError::new("A collection of type `Rule` is expected [ Rule, Rule ,Rule ]")
                })?
                .into_iter()
                .map(|rule| rule.into()),
        )))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<WasmCartridge> for Cartridge<Rule> {
    fn from(value: WasmCartridge) -> Self {
        value.0
    }
}
