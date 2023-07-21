use super::*;

mod captures;
mod traits;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Rule {
    content: Option<TakeRuleForExtend>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TakeRuleForExtend {
    pub str_with_type: RegexRaw,
    pub requirement: MatchRequirement,
    pub subrules: Option<Subrules>,
    pub counter: Option<Counter>,
    pub mod_match: ModeMatch,
    pub duplicate_matches: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexRaw {
    DefaultR(Box<str>),
    FancyR(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBeFound,
}

#[derive(Debug, Clone)]
pub struct Subrules {
    pub simple_rules: Option<SimpleRules>,
    pub complex_rules: Option<Vec<Rule>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Counter {
    Only(usize),
    MoreThan(usize),
    LessThan(usize),
}

#[derive(Debug, Clone)]
pub enum ModeMatch {
    AllRulesForAllMatches,
    AllRulesForAtLeastOneMatch,
    AtLeastOneRuleForAllMatches,
    AtLeastOneRuleForAtLeastOneMatch,
}

#[derive(Debug, Clone)]
pub struct SimpleRules {
    pub all_rules: HashSet<Rule>,
    pub regex_set: regex::RegexSet,
}
