use super::*;
impl<T> Cartridge<T>
where
    T: RuleBase + RuleModifiers<RuleType = T> + Default,
{
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.root_rule = self.root_rule.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.root_rule = self.root_rule.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }
    
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.root_rule = self
            .root_rule
            .mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }
}
