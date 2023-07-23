use crate::core::rules::traits::{RuleBase, RuleBytesExtendBase};

use super::*;
impl RuleBase for RuleBytes {
    type TakeRuleType = TakeRuleBytesForExtend;
    /// Use for direct access to the structure body
    fn content_unchecked(&self) -> &TakeRuleBytesForExtend {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    fn content_mut_unchecked(&mut self) -> &mut TakeRuleBytesForExtend {
        self.0.as_mut().expect(ERR_OPTION)
    }
}

impl RuleBytesExtendBase for RuleBytes {
    /// Get selected rules from `RegexSet`
    fn get_selected_rules(regex_set: &regex::bytes::RegexSet, text_bytes: &[u8]) -> Vec<usize> {
        regex_set.matches(text_bytes).iter().collect()
    }
}
