mod modifiers;
use super::RuleBytes as RustRuleBytes;
use super::*;
#[wasm_bindgen]
pub struct RuleBytes(RustRuleBytes);

#[wasm_bindgen]
impl RuleBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self(RustRuleBytes::new(pattern, requirement.into()))
    }
}
