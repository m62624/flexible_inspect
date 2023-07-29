use super::*;
use crate::core::rules::traits::RuleModifiers;
use crate::export_lang::python_version::rules::traits::PyRuleModifiers;
use pyo3::PyResult;

impl PyRuleModifiers for PyRule {
    type PyRuleType = PyRule;

    fn _counter_is_equal(&mut self, count: usize) -> Self::PyRuleType {
        let rule_with_counter = self.to_rust().counter_is_equal(count);
        self.0 = rule_with_counter;
        std::mem::take(self)
    }

    fn _counter_more_than(&mut self, count: usize) -> Self::PyRuleType {
        let rule_with_counter = self.to_rust().counter_more_than(count);
        self.0 = rule_with_counter;
        std::mem::take(self)
    }

    fn _counter_less_than(&mut self, count: usize) -> Self::PyRuleType {
        let rule_with_counter = self.to_rust().counter_less_than(count);
        self.0 = rule_with_counter;
        std::mem::take(self)
    }

    fn _mode_all_rules_for_at_least_one_match(&mut self) -> Self::PyRuleType {
        let rule_new_mode = self.to_rust().mode_all_rules_for_at_least_one_match();
        self.0 = rule_new_mode;
        std::mem::take(self)
    }

    fn _mode_at_least_one_rule_for_all_matches(&mut self) -> Self::PyRuleType {
        let rule_new_mode = self.to_rust().mode_at_least_one_rule_for_all_matches();
        self.0 = rule_new_mode;
        std::mem::take(self)
    }

    fn _mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::PyRuleType {
        let rule_new_mode = self
            .to_rust()
            .mode_at_least_one_rule_for_at_least_one_match();
        self.0 = rule_new_mode;
        std::mem::take(self)
    }
}

#[pymethods]
impl PyRule {
    /// modifier for extending the rule with nested rules
    /// ( **by default, all rules must pass every match check** )
    pub fn extend(&mut self, py: Python, nested_rules: PyObject) -> PyResult<Self> {
        let collect = self._to_rust_for_extend::<PyRule>(py, nested_rules, "Rule")?;
        self.0 = self.0.extend(collect);
        Ok(std::mem::take(self))
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
