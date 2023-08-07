use super::*;

#[pymethods]
impl PyRuleBytes {
    pub fn extend(&mut self, nested_rules: Vec<PyRuleBytes>) -> PyResult<Self> {
        self.0 = self
            .0
            .extend(nested_rules.into_iter().map(|rule| rule.into()));
        todo!()
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

    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }
}
