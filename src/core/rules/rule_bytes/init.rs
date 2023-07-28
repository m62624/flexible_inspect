use super::*;
use crate::init_logger;

impl TakeRuleBytesForExtend {
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self {
            str_bytes: if regex::bytes::Regex::new(&pattern).is_ok() {
                pattern.into_boxed_str()
            } else {
                panic!("`{}` regular expression is incorrect", pattern);
            },
            subrules_bytes: None,
            general_modifiers: GeneralModifiers::new(requirement),
        }
    }
}

impl SimpleRulesBytes {
    /// Constructor for creating simple rules and `regexset`
    /*
    We use `unwrap` instead of `?` because we guarantee that if there are `Rule` that are in this constructor, they have already passed regular expression validity checks
     */
    pub fn new(all_rules: IndexSet<RuleBytes>) -> Self {
        Self {
            regex_set: regex::bytes::RegexSet::new(&all_rules).unwrap(),
            all_rules,
        }
    }
}
