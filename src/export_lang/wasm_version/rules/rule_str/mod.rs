mod modifiers;
use super::*;

#[wasm_bindgen(js_name = Rule)]
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
pub struct WasmRule(Rule);

#[wasm_bindgen(js_class = Rule)]
impl WasmRule {
    pub fn start_build(pattern: String, requirement: MatchRequirement) -> Self {
        console_error_panic_hook::set_once();
        Self(Rule::new(pattern, requirement.into()))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}
