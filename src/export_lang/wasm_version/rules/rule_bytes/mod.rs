mod modifiers;

use super::*;
#[wasm_bindgen(js_name = RuleBytes)]
#[derive(Serialize, Deserialize, Default)]
pub struct WasmRuleBytes(RuleBytes);

#[wasm_bindgen]
impl WasmRuleBytes {
    pub fn start_build(pattern: String, requirement: MatchRequirement) -> Self {
        Self(RuleBytes::new(pattern, requirement.into()))
    }
    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}
