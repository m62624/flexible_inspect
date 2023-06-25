use super::*;
mod captures;
mod getters;
#[derive(Debug, Clone)]
pub enum RuleType {
    DefaultRegex(Box<str>),
    FancyRegex(Box<str>),
}

#[derive(Debug, Clone)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBeFound,
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Rule {
    rule_raw: Option<RuleType>,
    requirement: Option<MatchRequirement>,
    subrules: Option<Vec<Rule>>,
}
