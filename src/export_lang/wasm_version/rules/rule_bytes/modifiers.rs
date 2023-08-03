use super::*;
use crate::core::rules::traits::RuleModifiers;

impl WasmRuleModifiers for WasmRuleBytes {
    type WasmRuleType = WasmRuleBytes;
    type RustRuleType = RuleBytes;
    fn _counter_is_equal(&mut self, count: usize) -> Self::WasmRuleType {
        self.0 = self.0.counter_is_equal(count);
        std::mem::take(self)
    }

    fn _counter_more_than(&mut self, count: usize) -> Self::WasmRuleType {
        self.0 = self.0.counter_more_than(count);
        std::mem::take(self)
    }

    fn _counter_less_than(&mut self, count: usize) -> Self::WasmRuleType {
        self.0 = self.0.counter_less_than(count);
        std::mem::take(self)
    }

    fn _mode_all_rules_for_at_least_one_match(&mut self) -> Self::WasmRuleType {
        self.0 = self.0.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    fn _mode_at_least_one_rule_for_all_matches(&mut self) -> Self::WasmRuleType {
        self.0 = self.0.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    fn _mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::WasmRuleType {
        self.0 = self.0.mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }
}

#[wasm_bindgen(js_class = RuleBytes)]
impl WasmRuleBytes {
    pub fn extend(&mut self, rules: Vec<JsValue>) -> WasmRuleBytes {
        // self.0 = self.0.extend(Self::_to_rust_for_extend(rules)?);
        self.0 = self.0.extend(
            rules
                .into_iter()
                .map(|rule_js| {
                    serde_wasm_bindgen::from_value::<WasmRuleBytes>(rule_js)
                        .and_then(|x| Ok(x.0))
                        .expect("\nRule loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for `completion`.\n2) You can only use the `RuleBytes` type for the root
                        ")
                })
                .collect::<Vec<RuleBytes>>(),
        );
        std::mem::take(self)
    }
    /// modifier to set the match counter, condition counter == match
    pub fn counter_is_equal(&mut self, count: usize) -> Self {
        self._counter_is_equal(count)
    }

    /// modifier to set the match counter, condition counter >= match
    pub fn counter_more_than(&mut self, count: usize) -> Self {
        self._counter_more_than(count)
    }

    /// modifier to set the match counter, condition counter <= match
    pub fn counter_less_than(&mut self, count: usize) -> Self {
        self._counter_less_than(count)
    }

    /// modifier to change the rule matching mode,
    /// `all rules` must pass the test for at least `one match`
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self._mode_all_rules_for_at_least_one_match()
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for `all matches`
    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self._mode_at_least_one_rule_for_all_matches()
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for at least `one match`
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self._mode_at_least_one_rule_for_at_least_one_match()
    }
}
