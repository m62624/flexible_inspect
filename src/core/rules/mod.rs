// =======================================================
pub mod captures;
mod init;
pub mod next;
pub mod rule_bytes;
pub mod rule_str;
pub mod traits;
// =======================================================
use super::*;
use crate::Rule;
use indexmap::IndexSet;
use std::hash::Hash;
// =======================================================

/// The struct for sorting all nested rules
pub struct SlisedRules {
    /// `IndexSet` provides access to items in `O(1)` time on average when using the contains method.
    pub simple_rules: IndexSet<Rule>,
    pub complex_rules: Vec<Rule>,
}

/// A Structure for common `Rule` modifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralModifiers {
    pub requirement: MatchRequirement,
    pub counter: Option<Counter>,
    pub mod_match: ModeMatch,
}

/// A structure that defines what action is required when finding regular expression matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBeFound,
}

/// A structure defining the operation mode of the validator subrules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModeMatch {
    AllRulesForAllMatches,
    AllRulesForAtLeastOneMatch,
    AtLeastOneRuleForAllMatches,
    AtLeastOneRuleForAtLeastOneMatch,
}

/// A structure for realization of modifier-counters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Counter {
    Only(usize),
    MoreThan(usize),
    LessThan(usize),
}
