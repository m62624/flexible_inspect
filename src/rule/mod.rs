mod extend;
mod getters;
mod init;
mod runner;
pub mod slice;
use super::*;

/// --> ExceptionContainer
#[pyclass]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Rule {
    str_raw: Option<RegexRaw>,
    requirement: Option<MatchRequirement>,
    subrules: Option<Subrules>,
}

/// --> Rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexRaw {
    DefaultR(Box<str>),
    FancyR(Box<str>),
}

/// --> Rule
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl PartialEq for Subrules {
    fn eq(&self, other: &Self) -> bool {
        self.default_rgx_vec == other.default_rgx_vec && self.fancy_rgx_vec == other.fancy_rgx_vec
    }
}

impl Eq for Subrules {}
