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
                RegexRaw::BytesRegex(pattern.into_boxed_str())
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
    pub fn new(
        smr_must_be_found: IndexSet<RuleBytes>,
        smr_must_not_be_found_with_subrules: IndexSet<RuleBytes>,
        smr_must_not_be_found_without_subrules: IndexSet<RuleBytes>,
    ) -> Self {
        let rgxst = regex::bytes::RegexSet::new(
            smr_must_be_found
                .iter()
                .chain(&smr_must_not_be_found_with_subrules)
                .chain(&smr_must_not_be_found_without_subrules),
        )
        .unwrap_or_else(|err| {
            let err_msg = format!("`{}` regular expression is incorrect", err);
            error!("{}", err_msg);
            panic!("{}", err_msg);
        });
        Self {
            smr_must_be_found,
            smr_must_not_be_found_with_subrules,
            smr_must_not_be_found_without_subrules,
            regex_set: RegexSetContainer { regex_set: rgxst },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.smr_must_be_found.is_empty()
            || self.smr_must_not_be_found_with_subrules.is_empty()
            || self.smr_must_not_be_found_without_subrules.is_empty()
    }
}

impl RangeBytes {
    pub fn new(range: range::RangeBoundaries, read_mode: ReadMode, range_mode: RangeMode) -> Self {
        Self {
            range,
            read_mode,
            range_mode,
        }
    }
}
