mod modifiers;
use super::Rule as RustRule;
use super::*;
#[wasm_bindgen]
pub struct Rule(RustRule);

#[wasm_bindgen]
impl Rule {
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self(RustRule::new(pattern, requirement.into()))
    }
}
