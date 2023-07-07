use super::*;

#[pymethods]
impl Rule {
    #[new]
    pub fn new(pattern: String, requirements: MatchRequirement) -> PyResult<Self> {
        Ok(Self {
            content: Some(TakeRuleForExtend::new(pattern, requirements)?),
        })
    }
}

impl TakeRuleForExtend {
    fn new(pattern: String, requirements: MatchRequirement) -> PyResult<Self> {
        Ok(Self {
            str_with_type: RegexRaw::new(pattern)?,
            requirement: requirements,
            subrules: None,
            counter: None,
            mod_match: ModeMatch::AllRulesForAllMatches,
        })
    }
}

impl RegexRaw {
    fn new(pattern: String) -> PyResult<Self> {
        if regex::Regex::new(&pattern).is_ok() {
            return Ok(RegexRaw::DefaultR(pattern.into_boxed_str()));
        } else if fancy_regex::Regex::new(&pattern).is_ok() {
            return Ok(RegexRaw::FancyR(pattern.into_boxed_str()));
        }
        Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "Expected `Regex` or `FancyRegex`, got `{}`",
            pattern
        )))
    }
}
impl Subrules {
    pub fn new(simple_rules: SimpleRules, complex_rules: Vec<Rule>) -> Self {
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
    pub fn new(all_rules: Vec<Rule>) -> Self {
        Self {
            regex_set: regex::RegexSet::new(
                &all_rules
                    .iter()
                    .filter(|rule| {
                        matches!(
                            rule.content_unchecked().requirement,
                            MatchRequirement::MustBeFound
                        )
                    })
                    .map(|rule| rule.as_ref())
                    .collect::<Vec<&str>>(),
            )
            .unwrap(),
            all_rules,
        }
    }
}
