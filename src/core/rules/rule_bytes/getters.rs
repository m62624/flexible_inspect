use super::SimpleRulesBytes;
use super::*;
use crate::core::rules::{
    captures::CaptureData,
    traits::{RuleBase, RuleBytesExtendBase},
};
impl RuleBase for RuleBytes {
    type TakeRuleType = TakeRuleBytesForExtend;
    type SubRulesType = SimpleRulesBytes;
    /// Use for direct access to the structure body
    fn content_unchecked(&self) -> &Self::TakeRuleType {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType {
        self.0.as_mut().expect(ERR_OPTION)
    }

    fn get_subrules(&self) -> Option<&Self::SubRulesType> {
        self.content_unchecked().subrules_bytes.as_ref()
    }

    fn get_requirement(&self) -> &MatchRequirement {
        &self.content_unchecked().general_modifiers.requirement
    }

    fn get_counter(&self) -> Option<Counter> {
        self.content_unchecked().general_modifiers.counter
    }
}

impl<'a> RuleBytesExtendBase<'a> for RuleBytes {
    /// Get selected rules from `RegexSet`
    fn get_selected_rules(regex_set: &regex::bytes::RegexSet, text_bytes: &[u8]) -> Vec<usize> {
        regex_set.matches(text_bytes).iter().collect()
    }

    fn find_captures(rule: &RuleBytes, text_bytes: &'a [u8]) -> CaptureData<'a> {
        captures::find_captures(rule, text_bytes)
    }
}
