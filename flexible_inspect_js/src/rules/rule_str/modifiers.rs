use super::*;

#[wasm_bindgen(js_class = "Rule")]
impl WasmRule {
    /// # Arguments:
    /// * `nested_rules` - \[ `Rule`, `Rule`, `Rule` \] (collection)
    ///
    /// modifier for extending the rule with nested rules
    ///
    /// ( **by default, all rules must pass every match check** )\
    /// In this mode, to which all additional rules apply (default mode for everyone).
    /// We check that for each match (text) all the rules will work.
    /// ## Operation scheme of the mode
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///      |   [123], [456], [789]
    ///      |___ Subrule ".+" (MustBeFound) ---> [123] -> [456] -> [789] -- TRUE
    ///      |                                      |       |        |
    ///      |___ Subrule "\[\d+\]" (MustBeFound) __|_______|________|
    ///
    /// ```
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
    /// In this mode, `all the sub-rule` should work for at least `one match`.
    /// If at least one sub-rule does not work on one of the matches, an error will be returned.
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError (Cartridge)
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///     |   [123], [456], [789]
    ///     |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE
    ///     |                                      |
    ///     |___ Subrule "\[\d+\]" (MustBeFound) __|
    ///     |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- ERROR
    /// ```
    pub fn all_r_for_any_m(&mut self) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.all_r_for_any_m());
        Ok(mem_self)
    }

    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least `one sub-rule` should work for `every match`. If no sub-rule works on one of the matches, an error will be returned.
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError (Cartridge)
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///     |   [123], [456], [789]
    ///     |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE -- [456] -- TRUE -- [789] -- TRUE
    ///     |                                      |               |                 |
    ///     |___ Subrule "\[\d+\]" (MustBeFound) __|_______________|_________________|
    ///     |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- TRUE (since other rules matched)
    /// ```
    pub fn any_r_for_all_m(&mut self) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_all_m());
        Ok(mem_self)
    }

    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least `one sub-rule` should work for at least `one match`. If no sub-rule works on one of the matches, an error will be returned.
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError (Cartridge)
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///     |   [123], [456], [789]
    ///     |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE
    ///     |                                      |
    ///     |___ Subrule "\[\d+\]" (MustBeFound) __|
    ///     |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- TRUE (since other rules matched for at least one match)
    /// ```

    pub fn any_r_for_any_m(&mut self) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_any_m());
        Ok(mem_self)
    }
}
