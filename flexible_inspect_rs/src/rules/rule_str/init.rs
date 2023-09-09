use super::*;
#[cfg(feature = "log_rust")]
use crate::logs::init_logger_with_offset;
use log::trace;

impl Rule {
    /// Constructor for creating `Rule`
    ///
    /// # Notes
    /// * By default, `all_rules_for_all_matches`
    /// * Please stick to *raw string literals* when creating regular expressions, without it your regular expression may behave differently
    /// ## Example:
    /// > * `r"d{3}."` - is the correct conversion to a regular expression
    /// > * `"d{3}."` - possible incorrect behavior
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    /// let rule1 = Rule::new(r"\d+", MatchRequirement::MustBeFound);
    /// let rule2 = Rule::new(r"\s+", MatchRequirement::MustNotBeFound);
    /// ```
    pub fn new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self {
        #[cfg(feature = "log_rust")]
        {
            init_logger_with_offset(0);
        }
        Self(TakeRuleForExtend::new(pattern.into(), requirement))
    }
}
impl TakeRuleForExtend {
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self {
            str_with_type: RegexRaw::new(pattern),
            subrules: None,
            general_modifiers: GeneralModifiers::new(requirement),
        }
    }
}

impl RegexRaw {
    fn new(pattern: String) -> Self {
        if regex::Regex::new(&pattern).is_ok() {
            trace!("`{}` - `Default Regex` category is set\n* Lookahead and Lookbehind references - FALSE\n* Rust RegexSet - TRUE",pattern.yellow());
            RegexRaw::DefaultRegex(pattern.into_boxed_str())
        } else if fancy_regex::Regex::new(&pattern).is_ok() {
            trace!("`{}` - `Fancy Regex` category is set\n* Lookahead and Lookbehind references - TRUE\n* Rust RegexSet - FALSE",pattern.yellow());
            RegexRaw::FancyRegex(pattern.into_boxed_str())
        } else {
            let err_msg = format!("`{}` regular expression is incorrect", pattern);
            error!("{}", err_msg);
            panic!("{}", err_msg);
        }
    }
}

impl Subrules {
    pub fn new(simple_rules: SimpleRules, complex_rules: IndexSet<Rule>) -> Self {
        Self {
            simple_rules: match !simple_rules.is_empty() {
                true => Some(simple_rules),
                false => None,
            },
            complex_rules: match !complex_rules.is_empty() {
                true => Some(complex_rules),
                false => None,
            },
        }
    }
}

impl SimpleRules {
    /// Constructor for creating simple rules and `regexset`
    /*
    We use `unwrap` instead of `?` because we guarantee that if there are `Rule` that are in this constructor, they have already passed regular expression validity checks
     */

    pub fn new(
        smr_must_be_found: IndexSet<Rule>,
        smr_must_not_be_found_with_subrules: IndexSet<Rule>,
        smr_must_not_be_found_without_subrules: IndexSet<Rule>,
    ) -> Self {
        let rgxst = regex::RegexSet::new(
            smr_must_be_found
                .iter()
                .chain(&smr_must_not_be_found_with_subrules)
                .chain(&smr_must_not_be_found_without_subrules),
        )
        .unwrap();
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

impl RangeStr {
    pub fn new(range: range::RangeBoundaries, range_mode: RangeMode) -> Self {
        Self { range, range_mode }
    }
}
