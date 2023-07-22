use super::*;

impl Rule {
    pub fn new<T: ToString>(pattern: T, requirement: MatchRequirement) -> Self {
        Self(Some(TakeRuleForExtend::new(
            pattern.to_string(),
            requirement,
        )))
    }
}

impl TakeRuleForExtend {
    fn new(pattern: String, requirement: MatchRequirement) -> Self {
        Self {
            str_with_type: RegexRaw::new(pattern),
            mutable_modifiers: MutableModifiers::new(requirement),
        }
    }
}

impl MutableModifiers {
    fn new(requirement: MatchRequirement) -> Self {
        Self {
            requirement,
            subrules: None,
            counter: None,
            mod_match: ModeMatch::AllRulesForAllMatches,
        }
    }
}

impl RegexRaw {
    fn new(pattern: String) -> Self {
        if regex::Regex::new(&pattern).is_ok() {
            return RegexRaw::DefaultRegex(pattern.into_boxed_str());
        } else if fancy_regex::Regex::new(&pattern).is_ok() {
            return RegexRaw::FancyRegex(pattern.into_boxed_str());
        } else if regex::bytes::Regex::new(&pattern).is_ok() {
            return RegexRaw::RegexBytes(pattern.into_boxed_str());
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
