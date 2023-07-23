// =======================================================
mod init;
pub mod rule_bytes;
pub mod rule_str;
// =======================================================
use super::*;
use indexmap::IndexSet;
use std::hash::Hash;
// =======================================================

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
