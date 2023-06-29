mod extend;
mod getters;
mod init;
pub mod slice;
use super::*;

/// --> ExceptionContainer
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Rule {
    str_raw: Option<RegexRaw>,
    requirement: Option<MatchRequirement>,
    subrules: Option<Subrules>,
}

/// --> Rule
#[derive(Debug, Clone)]
pub enum RegexRaw {
    DefaultR(Box<str>),
    FancyR(Box<str>),
}

/// --> Rule
#[pyclass]
#[derive(Debug, Clone)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}

/// --> Rule
#[derive(Debug, Clone)]
pub struct Subrules {
    default_rgx_set: Option<regex::RegexSet>,
    default_rgx_vec: Option<Vec<Rule>>,
    fancy_rgx_vec: Option<Vec<Rule>>,
}
