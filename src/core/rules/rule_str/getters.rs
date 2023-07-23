use super::*;
use crate::core::rules::traits::{RuleBase, RuleExtendBase};

impl RuleBase for Rule {
    type TakeRuleType = TakeRuleForExtend;
    /// Use for direct access to the structure body
    fn content_unchecked(&self) -> &TakeRuleForExtend {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    fn content_mut_unchecked(&mut self) -> &mut TakeRuleForExtend {
        self.0.as_mut().expect(ERR_OPTION)
    }
}

impl RuleExtendBase for Rule {
    /// Get selected rules from `RegexSet`
    fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect()
    }
}
