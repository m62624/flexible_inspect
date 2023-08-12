use super::*;

#[wasm_bindgen(js_class = "Rule")]
impl WasmRule {
    pub fn extend(&mut self, nested_rules: JsValue) -> Result<WasmRule, JsValue> {
        self.0 = self.0.extend(
            serde_wasm_bindgen::from_value::<Vec<WasmRule>>(nested_rules)
                .map_err(|_| {
                    JsValue::from_str("`Rule` loading error, possible causes:\n1) You may have forgotten to specify `finish_build()` for completion.\n2) You can only use the `Rule` ( [ Rule, Rule, Rule ] )")
                })?
                .into_iter()
                .map(|rule| rule.into()),
        );
        Ok(std::mem::take(self))
    }

    pub fn counter_is_equal(&mut self, count: usize) -> Self {
        self.0 = self.0.counter_is_equal(count);
        std::mem::take(self)
    }

    pub fn counter_more_than(&mut self, count: usize) -> Self {
        self.0 = self.0.counter_more_than(count);
        std::mem::take(self)
    }

    pub fn counter_less_than(&mut self, count: usize) -> Self {
        self.0 = self.0.counter_less_than(count);
        std::mem::take(self)
    }

    pub fn all_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.all_r_for_any_m();
        std::mem::take(self)
    }

    pub fn any_r_for_all_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_all_m();
        std::mem::take(self)
    }

    pub fn any_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_any_m();
        std::mem::take(self)
    }
}
