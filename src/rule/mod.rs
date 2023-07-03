mod getters;
mod init;
mod modifiers;
mod next;
mod runner;
mod slice;
mod traits;
use super::*;

/// --> ExceptionContainer
#[pyclass]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Rule {
    content: Option<TakeRuleForExtend>,
}

/// --> Rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TakeRuleForExtend {
    pub str_with_type: RegexRaw,
    pub requirement: MatchRequirement,
    pub subrules: Option<Subrules>,
    pub counter: Option<Counter>,
}

/// --> TakeRuleForExtend
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexRaw {
    DefaultR(Box<str>),
    FancyR(Box<str>),
}

/// --> TakeRuleForExtend
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}

/// --> TakeRuleForExtend
/// --> Cartridge
#[derive(Debug, Clone)]
pub struct Subrules {
    pub simple_rules: Option<SimpleRules>,
    pub complex_rules: Option<Vec<Rule>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Counter {
    Only(usize),
    /// включительно
    MoreThan(usize),
    /// включительно
    LessThan(usize),
}

/// --> Subrules
#[derive(Debug, Clone)]
pub struct SimpleRules {
    pub all_rules: Vec<Rule>,
    pub regex_set: regex::RegexSet,
}
