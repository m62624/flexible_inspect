mod modifiers;
use super::*;

#[wasm_bindgen(js_name = "RuleBytes")]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WasmRuleBytes(RuleBytes);

#[wasm_bindgen(js_class = "RuleBytes")]
impl WasmRuleBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: js_sys::RegExp, requirement: WasmMatchRequirement) -> Self {
        console_error_panic_hook::set_once();
        Self(RuleBytes::new(pattern.source(), requirement.into()))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<WasmRuleBytes> for RuleBytes {
    fn from(value: WasmRuleBytes) -> Self {
        value.0
    }
}
