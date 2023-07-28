use super::*;
use crate::core::rules::traits::RuleBase;

impl RuleBase for Rule {
    type TakeRuleType = TakeRuleForExtend;
    type SubRulesType = Subrules;
    type RegexSet = regex::RegexSet;
    type RuleType = Rule;

    fn _new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self {
        Rule::new(pattern, requirement)
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
        self.content_unchecked().str_with_type.as_ref()
    }
    fn get_subrules(&self) -> Option<&Self::SubRulesType> {
        self.content_unchecked().subrules.as_ref()
    }

    fn get_simple_rules(&self) -> Option<(&IndexSet<Self::RuleType>, &Self::RegexSet)> {
        if let Some(subrules) = self.get_subrules() {
            if let Some(simple_rules) = &subrules.simple_rules {
                return Some((&simple_rules.all_rules, &simple_rules.regex_set));
            }
        }
        None
    }

    fn get_complex_rules(&self) -> Option<&IndexSet<Self::RuleType>> {
        if let Some(subrules) = self.get_subrules() {
            if let Some(complex_rules) = &subrules.complex_rules {
                return Some(&complex_rules);
            }
        }
        None
    }
}
