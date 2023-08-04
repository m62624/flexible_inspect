use super::*;

#[wasm_bindgen(js_name = CartridgeBytes)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub struct WasmCartridgeBytes(pub(crate) Cartridge<RuleBytes>);

#[wasm_bindgen(js_class = CartridgeBytes)]
impl WasmCartridgeBytes {
    pub fn start_build(
        id: i32,
        message: String,
        root_rules: Vec<JsValue>,
    ) -> Result<WasmCartridgeBytes, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self(Cartridge::new(
            id,
            message,
            WasmRuleBytes::_to_rust_for_extend(root_rules, format!("\n` (CartridgeBytes) RuleBytes` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `RuleBytes` (collection format) type for the `CartridgeBytes`").as_str(),"\nYou must specify at least one rule for the `CartridgeBytes` (collection format) [ RuleBytes, RuleBytes, RuleBytes ]")?,
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

    /// Get the `error code`
    pub fn get_id(&self) -> i32 {
        self.0.id
    }

    /// Get an `error message` with data
    pub fn get_message(&self) -> String {
        self.0.message.clone()
    }
}
