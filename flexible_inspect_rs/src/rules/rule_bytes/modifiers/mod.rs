mod extend;
use log::debug;

use super::*;
use crate::rules::traits::{RuleBase, RuleModifiers};

impl RuleModifiers for RuleBytes {
    type RuleType = RuleBytes;
    /// a method for extending the rule with nested rules
    fn extend<R: IntoIterator<Item = Self::RuleType>>(
        &mut self,
        nested_rules: R,
    ) -> Self::RuleType {
        extend::extend(self, nested_rules)
    }

    fn counter_is_equal(&mut self, count: usize) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.counter = Some(Counter::Only(count));
        debug!(
            "the `counter_is_equal ({})` modifier is applied to rule ({}, {:#?})",
            count,
            self.get_str(),
            self.get_requirement()
        );
        std::mem::take(self)
    }

    fn counter_more_than(&mut self, count: usize) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.counter = Some(Counter::MoreThan(count));
        debug!(
            "the `counter_more_than ({})` modifier is applied to rule ({}, {:#?})",
            count,
            self.get_str(),
            self.get_requirement()
        );
        std::mem::take(self)
    }

    fn counter_less_than(&mut self, count: usize) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.counter = Some(Counter::LessThan(count));
        debug!(
            "the `counter_less_than ({})` modifier is applied to rule ({}, {:#?})",
            count,
            self.get_str(),
            self.get_requirement()
        );
        std::mem::take(self)
    }

    fn mode_all_rules_for_at_least_one_match(&mut self) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.mod_match =
            ModeMatch::AllRulesForAtLeastOneMatch;
        debug!(
            "the `mode_all_rules_for_at_least_one_match` modifier is applied to rule ({}, {:#?})",
            self.get_str(),
            self.get_requirement()
        );
        std::mem::take(self)
    }

    fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.mod_match =
            ModeMatch::AtLeastOneRuleForAllMatches;
        debug!(
            "the `mode_at_least_one_rule_for_all_matches` modifier is applied to rule ({}, {:#?})",
            self.get_str(),
            self.get_requirement()
        );
        std::mem::take(self)
    }

    fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.mod_match =
            ModeMatch::AtLeastOneRuleForAtLeastOneMatch;
        debug!(
            "the `mode_at_least_one_rule_for_at_least_one_match` modifier is applied to rule ({}, {:#?})",
            self.get_str(),
            self.get_requirement()
        );
        std::mem::take(self)
    }
}
