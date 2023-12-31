use super::*;
#[cfg(feature = "log_rust")]
use crate::logs::init_logger_with_offset;
use log::trace;

impl RuleBytes {
    /// Constructor for creating `RuleBytes`
    ///
    /// # Notes
    /// * By default, `all_rules_for_all_matches`
    /// * Please stick to *raw string literals* when creating regular expressions, without it your regular expression may behave differently
    /// ## Example
    /// > * `r"d{3}."` - is the correct conversion to a regular expression
    /// > * `"d{3}."` - possible incorrect behavior
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    /// let rule1 = RuleBytes::new(r"\d+", MatchRequirement::MustBeFound);
    /// let rule2 = RuleBytes::new(r"\s+", MatchRequirement::MustNotBeFound);
    /// ```
    pub fn new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self {
        #[cfg(feature = "log_rust")]
        {
            init_logger_with_offset(0);
        }
        Self(TakeRuleBytesForExtend::new(pattern.into(), requirement))
    }
}

impl TakeRuleBytesForExtend {
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self {
            str_bytes: if regex::bytes::Regex::new(&pattern).is_ok() {
                trace!(
                    "'{}' - regex category for byte validation is set",
                    pattern.yellow()
                );
                pattern.into_boxed_str()
            } else {
                let err_msg = format!("`{}` regular expression is incorrect", pattern);
                error!("{}", err_msg);
                panic!("{}", err_msg);
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
            regex_set: RegexSetContainer {
                regex_set: regex::bytes::RegexSet::new(&all_rules).unwrap(),
            },
            all_rules,
        }
    }
}
