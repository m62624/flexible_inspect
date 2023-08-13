// =======================================================
mod init;
pub mod next;
pub mod rule_bytes;
pub mod rule_str;
pub mod runner;
pub mod traits;
// =======================================================
use super::*;
use crate::prelude::Rule;
use indexmap::IndexSet;
use log::error;
use std::collections::HashMap;
use std::hash::Hash;
// =======================================================
/// This is error message when `Rule|RuleBytes` is empty
const ERR_OPTION: &str = "\nThe body of `Rule` is missing (inside Rule is the value None), you may have used modifiers separately from initializations, they take the value (ownership) of `Rule` (std::mem::take) and return the already modified version (specify the modifier in the same place where you initialize `Rule|RuleBytes`).\n";
/// This is reserved standard value for error filling
pub const DEFAULT_CAPTURE: &str = "main_capture";
// =======================================================

/// The struct for sorting all nested rules
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SlisedRules {
    /// The rules are in the `IndexSet` collection to preserve
    /// the order of the rules during index retrieval from the `RegexSet` and to avoid duplicate rules
    pub simple_rules: IndexSet<Rule>,
    pub complex_rules: IndexSet<Rule>,
}

/// A Structure for common `Rule` modifiers
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralModifiers {
    pub requirement: MatchRequirement,
    pub counter: Option<Counter>,
    pub mod_match: ModeMatch,
}

/// The structure that defines what action is required when finding regular expression matches.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchRequirement {
    /// the match must be found
    MustBeFound,
    /// the match must not be found
    MustNotBeFound,
}

/// A structure defining the operation mode of the validator subrules.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModeMatch {
    /// all rules must pass the test for all matches
    AllRulesForAllMatches,
    /// all rules must pass the test for at least one match
    AllRulesForAtLeastOneMatch,
    /// at least one rule must pass the test for all matches
    AtLeastOneRuleForAllMatches,
    /// at least one rule must pass the test for at least one match
    AtLeastOneRuleForAtLeastOneMatch,
}

/// A structure for realization of modifier-counters
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Counter {
    /// counter == match
    Only(usize),
    /// counter >= match
    MoreThan(usize),
    /// counter <= match
    LessThan(usize),
}

/// A structure that stores all the data for processing the capture
#[derive(Debug)]
pub struct CaptureData<T: PartialEq + Eq + Hash> {
    pub text_for_capture: IndexSet<T>,
    pub hashmap_for_error: HashMap<String, String>,
    pub counter_value: usize,
}
