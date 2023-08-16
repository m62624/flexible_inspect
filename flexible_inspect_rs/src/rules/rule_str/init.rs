use super::*;
#[cfg(feature = "log_rust")]
use crate::logs::init_logger_with_offset;
use log::trace;

impl Rule {
    /// Constructor for creating `Rule`
    ///
    /// # Notes
    /// * Please stick to *raw string literals* when creating regular expressions, without it your regular expression may behave differently
    /// ## Example:
    /// > * `r"d{3}."` - is the correct conversion to a regular expression
    /// > * `"d{3}."` - possible incorrect behavior
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    /// let rule1 = Rule::new(r"\d+", MatchRequirement::MustBeFound);
    /// let rule2 = Rule::new(r"\s+", MatchRequirement::MustNotBeFound);
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
            init_logger_with_offset(0);
        }
        Self(Some(TakeRuleForExtend::new(pattern.into(), requirement)))
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
            simple_rules: match !simple_rules.all_rules.is_empty() {
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
    pub fn new(all_rules: IndexSet<Rule>) -> Self {
        Self {
            regex_set: RegexSetContainer {
                regex_set: regex::RegexSet::new(&all_rules).unwrap(),
            },
            all_rules,
        }
    }
}
