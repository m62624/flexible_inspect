use super::{rule_str::RegexRaw, traits::IntoSpecificCaptureType, *};
use crate::prelude::Rule;

impl GeneralModifiers {
    pub fn new(requirement: MatchRequirement) -> Self {
        Self {
            requirement,
            counter: None,
            mode_match: ModeMatch::AllRulesForAllMatches,
            range: None,
            save_duplicates: false,
        }
    }
}

impl SlisedRules {
    /// The method for sorting all nested rules
    pub fn new<T: IntoIterator<Item = Rule>>(all_rules: T) -> SlisedRules {
        // smr - simple rules
        // cmr - complex rules

        // Based on this, we validate through `RegexSet` items,
        // if we found less items than there are in the collections, then the validation failed
        let mut smr_must_be_found = IndexSet::new();
        // Based on this, we simply check with `RegexSet`.
        let mut smr_must_not_be_found_with_subrules = IndexSet::new();
        // Based on this, we validate through `RegexSet` items, if we found even one item from this collection, then the validation failed
        let mut smr_must_not_be_found_without_subrules = IndexSet::new();
        let mut cmr = IndexSet::new();
        all_rules
            .into_iter()
            .for_each(|rule| match rule.0.str_with_type {
                RegexRaw::DefaultRegex(_) => match rule.0.general_modifiers.requirement {
                    MatchRequirement::MustBeFound => {
                        smr_must_be_found.insert(rule);
                    }
                    MatchRequirement::MustNotBeFound => match rule.0.subrules {
                        Some(subrules) => {
                            smr_must_not_be_found_with_subrules.insert(rule);
                        }
                        None => {
                            smr_must_not_be_found_without_subrules.insert(rule);
                        }
                    },
                },
                RegexRaw::FancyRegex(_) => {
                    cmr.insert(rule);
                }
            });

        SlisedRules {
            smr_must_be_found,
            smr_must_not_be_found_with_subrules,
            smr_must_not_be_found_without_subrules,
            cmr,
        }
    }

    /// A method for checking if there are any rules
    pub fn is_some(&self) -> bool {
        !self.smr_must_be_found.is_empty()
            || !self.smr_must_not_be_found_with_subrules.is_empty()
            || !self.smr_must_not_be_found_without_subrules.is_empty()
            || !self.cmr.is_empty()
    }
}

impl<'a, T: IntoSpecificCaptureType<'a>> CaptureData<'a, T> {
    pub fn is_some(&self) -> bool {
        match &self.text_for_capture {
            TypeStorageFormat::Single(value) => !value.0.is_empty(),
            TypeStorageFormat::Multiple(value) => !value.0.is_empty(),
        }
    }
}
