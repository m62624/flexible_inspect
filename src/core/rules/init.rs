use crate::Rule;

use super::{rule_str::RegexRaw, traits::RuleBase, *};

impl GeneralModifiers {
    pub fn new(requirement: MatchRequirement) -> Self {
        Self {
            requirement,
            counter: None,
            mod_match: ModeMatch::AllRulesForAllMatches,
        }
    }
}

impl SlisedRules {
    /// The method for sorting all nested rules
    pub fn new<T: IntoIterator<Item = Rule>>(all_rules: T) -> SlisedRules {
        /*
        We do not save directly via `insert` to `IndexSet` as we would lose the ordering,
        we need the order to properly check rules that are not in `RegexSet` when searching through the index.
        `RegexSet` is implemented in `simple_regex` and `bytes_regex`,
        so there is no need to follow the queue for `complex rules`
        */
        let mut o_simple_rules = Vec::new();
        let mut o_complex_rules = Vec::new();

        all_rules
            .into_iter()
            .for_each(|rule| match rule.content_unchecked().str_with_type {
                RegexRaw::DefaultRegex(_) => o_simple_rules.push(rule),
                RegexRaw::FancyRegex(_) => o_complex_rules.push(rule),
            });

        let o_simple_rules: IndexSet<_> = o_simple_rules.into_iter().collect();

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
