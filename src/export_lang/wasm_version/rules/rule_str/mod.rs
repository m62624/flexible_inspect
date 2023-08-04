mod modifiers;
use super::*;

#[wasm_bindgen(js_name = Rule)]
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
pub struct WasmRule(pub(crate) Rule);

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

impl From<WasmRule> for Rule {
    fn from(wasm_rule: WasmRule) -> Self {
        wasm_rule.0
    }
}