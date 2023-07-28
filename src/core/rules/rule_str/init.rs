use super::*;

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
            RegexRaw::DefaultRegex(pattern.into_boxed_str())
        } else if fancy_regex::Regex::new(&pattern).is_ok() {
            RegexRaw::FancyRegex(pattern.into_boxed_str())
        } else {
            panic!("`{}` regular expression is incorrect", pattern)
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
            regex_set: regex::RegexSet::new(&all_rules).unwrap(),
            all_rules,
        }
    }
}
