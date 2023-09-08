use super::SimpleRulesBytes;
use super::*;
use crate::rules::traits::RuleBase;

impl RuleBase for RuleBytes {
    type TakeRuleType = TakeRuleBytesForExtend;
    type SubRulesType = SimpleRulesBytes;
    type RuleType = RuleBytes;
    type RegexSet = regex::bytes::RegexSet;

    fn _new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self {
        RuleBytes::new(pattern, requirement)
    }

    

    fn get_requirement(&self) -> MatchRequirement {
        self.0.general_modifiers.requirement
    }

    fn get_counter(&self) -> Option<Counter> {
        self.0.general_modifiers.counter
    }

    fn get_mode_match(&self) -> &ModeMatch {
        &self.0.general_modifiers.mode_match
    }

    fn get_str(&self) -> &str {
        self.0.str_bytes.as_ref()
    }

    fn get_subrules(&self) -> Option<&Self::SubRulesType> {
        self.0.subrules_bytes.as_ref()
    }

    // fn get_simple_rules(&self) -> Option<(&IndexSet<Self::RuleType>, &Self::RegexSet)> {
    //     if let Some(value) = self.get_subrules() {
    //         return Some((&value.all_rules, &value.regex_set.regex_set));
    //     }
    //     None
    // }

    fn get_complex_rules(&self) -> Option<&IndexSet<Self::RuleType>> {
        None
    }

    fn get_range(&self) -> Option<&range::RangeFormat> {
        self.0.general_modifiers.range.as_ref()
    }

    fn get_save_duplicates(&self) -> bool {
        self.0.general_modifiers.save_duplicates
    }

    fn get_str_type(&self) -> &RegexRaw {
        &self.0.str_bytes
    }
}
