use super::*;
use crate::rules::traits::RuleBase;

impl RuleBase for Rule {
    type TakeRuleType = TakeRuleForExtend;
    type SubRulesType = Subrules;
    type RegexSet = regex::RegexSet;
    type RuleType = Rule;

    fn _new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self {
        Rule::new(pattern, requirement)
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
        self.0.str_with_type.as_ref()
    }
    fn get_subrules(&self) -> Option<&Self::SubRulesType> {
        self.0.subrules.as_ref()
    }



    fn get_complex_rules(&self) -> Option<&IndexSet<Self::RuleType>> {
        if let Some(subrules) = self.get_subrules() {
            if let Some(complex_rules) = &subrules.complex_rules {
                return Some(complex_rules);
            }
        }
        None
    }

    fn get_range(&self) -> Option<&range::RangeFormat> {
        self.0.general_modifiers.range.as_ref()
    }

    fn get_save_duplicates(&self) -> bool {
        self.0.general_modifiers.save_duplicates
    }
}
