use super::{rule_str::RegexRaw, traits::IntoSpecificCaptureType, *};
use crate::prelude::Rule;

impl GeneralModifiers {
    pub fn new(requirement: MatchRequirement) -> Self {
        Self {
            requirement,
            counter: None,
            mode_match: ModeMatch::AllRulesForAllMatches,
            range: None,
        }
    }
}

impl SlisedRules {
    /// The method for sorting all nested rules
    pub fn new<T: IntoIterator<Item = Rule>>(all_rules: T) -> SlisedRules {
        let mut o_simple_rules = IndexSet::new();
        let mut o_complex_rules = IndexSet::new();
        all_rules
            .into_iter()
            .for_each(|rule| match rule.0.str_with_type {
                RegexRaw::DefaultRegex(_) => {
                    o_simple_rules.insert(rule);
                }
                RegexRaw::FancyRegex(_) => {
                    o_complex_rules.insert(rule);
                }
            });
        SlisedRules {
            simple_rules: o_simple_rules,
            complex_rules: o_complex_rules,
        }
    }

    /// A method for checking if there are any rules
    pub fn is_some(&self) -> bool {
        !self.simple_rules.is_empty() || !self.complex_rules.is_empty()
    }
}

impl<'a, T: IntoSpecificCaptureType<'a>> CaptureData<'a, T> {
    pub fn is_some(&self) -> bool {
        !self.text_for_capture.is_empty()
    }
}
