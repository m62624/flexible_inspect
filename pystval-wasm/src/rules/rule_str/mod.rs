mod modifiers;
use super::*;

#[wasm_bindgen(js_name = "Rule")]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WasmRule(Rule);

#[wasm_bindgen(js_class = "Rule")]
impl WasmRule {
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: String, requirement: WasmMatchRequirement) -> Self {
        console_error_panic_hook::set_once();
        Self(Rule::new(pattern, requirement.into()))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<WasmRule> for Rule {
    fn from(value: WasmRule) -> Self {
        value.0
    }
}
