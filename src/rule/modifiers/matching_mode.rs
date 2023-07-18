use super::*;
/// Реалзиуем методы для использование из `Python`
#[pymethods]
impl Rule {
    /// All subrules should work successfully for all matches (text)
    pub fn mode_all_rules_for_all_matches(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::AllRulesForAllMatches;

        // ================= (LOG) =================
        debug!(
            "used the `mode_all_rules_for_all_matches` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );
        // =========================================

        std::mem::take(self)
    }

    /// All subrules should work successfully for at least one match (text)
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::AllRulesForAtLeastOneMatch;

        // ================= (LOG) =================
        debug!(
            "used the `mode_all_rules_for_at_least_one_match` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );
        // =========================================

        std::mem::take(self)
    }

    /// At least one rule should work successfully for all matches
    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::AtLeastOneRuleForAllMatches;

        // ================= (LOG) =================
        debug!(
            "used the `mode_at_least_one_rule_for_all_matches` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );
        // =========================================

        std::mem::take(self)
    }

    /// At least one rule should work successfully for at least one match
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::AtLeastOneRuleForAtLeastOneMatch;

        // ================= (LOG) =================
        debug!(
            "used the `mode_at_least_one_rule_for_at_least_one_match` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );

        // =========================================

        std::mem::take(self)
    }
}
