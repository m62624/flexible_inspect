use super::*;
#[cfg(feature = "log_rust")]
use crate::logs::init_logger;
use log::trace;

impl RuleBytes {
    /// Constructor for creating `RuleBytes`
    /// 
    /// # Notes
    /// * Please stick to *raw string literals* when creating regular expressions, without it your regular expression may behave differently
    /// ## Example
    /// > * `r"d{3}."` - is the correct conversion to a regular expression
    /// > * `"d{3}."` - possible incorrect behavior
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    /// let rule1 = RuleBytes::new(r"\d+", MatchRequirement::MustBeFound);
    /// let rule2 = RuleBytes::new(r"\s+", MatchRequirement::MustNotBeFound);
    /// ```
    /// * **By default, all rules must pass every match check**
    /// In this mode, to which all additional rules apply (default mode for everyone).
    /// We check that for each match (text) all the rules will work.
    /// ## Operation scheme of the mode
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///      |   [123], [456], [789]
    ///      |___ Subrule ".+" (MustBeFound) ---> [123] -> [456] -> [789] -- TRUE
    ///      |                                      |       |        |
    ///      |___ Subrule "\[\d+\]" (MustBeFound) __|_______|________|
    /// ```
    ///
    pub fn new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self {
        #[cfg(feature = "log_rust")]
        {
            init_logger();
        }
        Self(Some(TakeRuleBytesForExtend::new(
            pattern.into(),
            requirement,
        )))
    }
}

impl TakeRuleBytesForExtend {
    pub fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self {
            str_bytes: if regex::bytes::Regex::new(&pattern).is_ok() {
                trace!("'{}' - regex category for byte validation is set", pattern);
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
