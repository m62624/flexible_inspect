use super::*;

impl RuleModifiers for WasmRuleBytes {
    type RuleType = WasmRuleBytes;

    fn extend<R: IntoIterator<Item = Self::RuleType>>(
        &mut self,
        nested_rules: R,
    ) -> Self::RuleType {
        self.0 = self
            .0
            .extend(nested_rules.into_iter().map(|rule| rule.into()));
        std::mem::take(self)
    }

    fn counter_is_equal(&mut self, count: usize) -> Self::RuleType {
        self.0 = self.0.counter_is_equal(count);
        std::mem::take(self)
    }

    fn counter_more_than(&mut self, count: usize) -> Self::RuleType {
        self.0 = self.0.counter_more_than(count);
        std::mem::take(self)
    }

    fn counter_less_than(&mut self, count: usize) -> Self::RuleType {
        self.0 = self.0.counter_less_than(count);
        std::mem::take(self)
    }

    fn mode_all_rules_for_at_least_one_match(&mut self) -> Self::RuleType {
        self.0 = self.0.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self::RuleType {
        self.0 = self.0.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::RuleType {
        self.0 = self.0.mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }
}
