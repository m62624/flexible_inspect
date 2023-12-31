use super::*;
use crate::rules::traits::{RuleBase, RuleModifiers};
use log::debug;

impl RuleModifiers for Rule {
    type RuleType = Rule;
    fn extend<R: IntoIterator<Item = Self::RuleType>>(mut self, nested_rules: R) -> Self::RuleType {
        let sliced_rules = SlisedRules::new(nested_rules);
        if sliced_rules.is_some() {
            self.0.subrules = Some(Subrules::new(
                SimpleRules::new(sliced_rules.simple_rules),
                sliced_rules.complex_rules,
            ));
        }
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            "extend".bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }

    fn counter_is_equal(mut self, count: usize) -> Self::RuleType {
        self.0.general_modifiers.counter = Some(Counter::Only(count));
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            format!("counter_is_equal ({})", count).bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }

    fn counter_more_than(mut self, count: usize) -> Self::RuleType {
        self.0.general_modifiers.counter = Some(Counter::MoreThan(count));
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            format!("counter_more_than ({})", count).bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }

    fn counter_less_than(mut self, count: usize) -> Self::RuleType {
        self.0.general_modifiers.counter = Some(Counter::LessThan(count));
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            format!("counter_less_than ({})", count).bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }

    fn all_r_for_any_m(mut self) -> Self::RuleType {
        self.0.general_modifiers.mod_match = ModeMatch::AllRulesForAtLeastOneMatch;
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            "mode_all_rules_for_at_least_one_match".bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }

    fn any_r_for_all_m(mut self) -> Self::RuleType {
        self.0.general_modifiers.mod_match = ModeMatch::AtLeastOneRuleForAllMatches;
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            "mode_at_least_one_rule_for_all_matches".bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }

    fn any_r_for_any_m(mut self) -> Self::RuleType {
        self.0.general_modifiers.mod_match = ModeMatch::AtLeastOneRuleForAtLeastOneMatch;
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            "mode_at_least_one_rule_for_at_least_one_match".bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }
}
