mod modifiers;
use super::*;
#[wasm_bindgen(js_name = RuleBytes)]
#[derive(Default)]
pub struct WasmRuleBytes(RuleBytes);

#[wasm_bindgen]
impl WasmRuleBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self(RuleBytes::new(pattern, requirement.into()))
    }
}

