use super::*;

#[wasm_bindgen(js_name = Cartridge)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub struct WasmCartridge(pub(crate) Cartridge<Rule>);

#[wasm_bindgen(js_class = Cartridge)]
impl WasmCartridge {
    pub fn start_build(
        id: i64,
        message: String,
        root_rules: Vec<JsValue>,
    ) -> Result<WasmCartridge, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self(Cartridge::new(
            id,
            message,
            WasmRule::_to_rust_for_extend(root_rules, "Rule")?,
        )))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }

    /// modifier to change the rule matching mode,
    /// `all rules` must pass the test for at least `one match`
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for `all matches`
    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for at least `one match`
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }
}
