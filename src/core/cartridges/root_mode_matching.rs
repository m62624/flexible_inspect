use super::*;
impl<T> Cartridge<T>
where
    T: RuleBase + RuleModifiers<RuleType = T> + Default,
{
    /// modifier to change the rule matching mode,
    /// `all rules` must pass the test for at least `one match`
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.root_rule = self.root_rule.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for `all matches`
    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.root_rule = self.root_rule.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for at least `one match`
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.root_rule = self
            .root_rule
            .mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }
}
