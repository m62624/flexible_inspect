use super::*;
use crate::init_logger;

impl RuleBytes {
    pub fn new<T: ToString>(pattern: T, requirement: MatchRequirement) -> Self {
        init_logger();
        Self(Some(TakeRuleBytesForExtend::new(
            pattern.to_string(),
            requirement,
        )))
    }
}

impl TakeRuleBytesForExtend {
    fn new(pattern: String, requirement: MatchRequirement) -> Self {
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
