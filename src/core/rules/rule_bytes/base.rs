use super::SimpleRulesBytes;
use super::*;
use crate::core::rules::traits::RuleBase;
use crate::init_logger;

impl RuleBase for RuleBytes {
    type TakeRuleType = TakeRuleBytesForExtend;
    type SubRulesType = SimpleRulesBytes;
    type RuleType = RuleBytes;
    type RegexSet = regex::bytes::RegexSet;

    fn new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self {
        init_logger();
        Self(Some(TakeRuleBytesForExtend::new(
            pattern.into(),
            requirement,
        )))
    }

    /// Use for direct access to the structure body
    fn content_unchecked(&self) -> &Self::TakeRuleType {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType {
        self.0.as_mut().expect(ERR_OPTION)
    }

    fn get_requirement(&self) -> &MatchRequirement {
        &self.content_unchecked().general_modifiers.requirement
    }

    fn get_counter(&self) -> Option<Counter> {
        self.content_unchecked().general_modifiers.counter
    }

    fn get_mode_match(&self) -> &ModeMatch {
        &self.content_unchecked().general_modifiers.mod_match
    }

    fn get_str(&self) -> &str {
        &self.content_unchecked().str_bytes.as_ref()
    }

    fn get_subrules(&self) -> Option<&Self::SubRulesType> {
        self.content_unchecked().subrules_bytes.as_ref()
    }

    fn get_simple_rules(&self) -> Option<(&IndexSet<Self::RuleType>, &Self::RegexSet)> {
        if let Some(value) = self.get_subrules() {
            return Some((&value.all_rules, &value.regex_set));
        }
        None
    }

    fn get_complex_rules(&self) -> Option<&IndexSet<Self::RuleType>> {
        None
    }
}
