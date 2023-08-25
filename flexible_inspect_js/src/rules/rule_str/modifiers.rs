use super::*;

#[wasm_bindgen(js_class = "Rule")]
impl WasmRule {
    /// # Arguments:
    /// * `nested_rules` - \[ `Rule`, `Rule`, `Rule` \] (collection)
    ///
    /// modifier for extending the rule with nested rules
    ///
    /// ( **by default, `all_rules_for_all_matches`** )\
    /// In this mode, all rules must be tested for all matches
    pub fn extend(&mut self, nested_rules: JsValue) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        let nested_rules = serde_wasm_bindgen::from_value::<Vec<Rule>>(nested_rules)
            .map_err(|_| JsValue::from_str(ERR_OPTION_RULE))?;
        mem_self.0 = mem_self.0.map(|rule| rule.extend(nested_rules));
        Ok(mem_self)
    }

    /// modifier to set the match counter, condition `counter == match`
    pub fn counter_is_equal(&mut self, count: usize) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.counter_is_equal(count));
        Ok(mem_self)
    }
    /// modifier to set the match counter, condition `counter >= match`
    pub fn counter_more_than(&mut self, count: usize) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.counter_more_than(count));
        Ok(mem_self)
    }
    /// modifier to set the match counter, condition `counter <= match`
    pub fn counter_less_than(&mut self, count: usize) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.counter_less_than(count));
        Ok(mem_self)
    }

    /// modifier to change the rule matching mode.
    ///
    /// In this mode, all rules must pass the test for at least one match
    pub fn all_r_for_any_m(&mut self) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.all_r_for_any_m());
        Ok(mem_self)
    }

    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass the test for all matches.
    pub fn any_r_for_all_m(&mut self) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_all_m());
        Ok(mem_self)
    }

    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass at least one match check
    pub fn any_r_for_any_m(&mut self) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_any_m());
        Ok(mem_self)
    }
}
