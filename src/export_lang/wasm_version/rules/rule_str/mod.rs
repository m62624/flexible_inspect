mod modifiers;
use super::*;

#[wasm_bindgen(js_name = Rule)]
#[derive(Serialize, Deserialize, Default)]
pub struct WasmRule(Rule);

#[wasm_bindgen]
impl WasmRule {
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self(Rule::new(pattern, requirement.into()))
    }
}
