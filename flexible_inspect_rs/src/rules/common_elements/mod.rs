use self::range::RangeFormat;

use super::*;
pub mod range;
pub mod str_to_number;
// =======================================================
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
    pub range: Option<RangeFormat>,
}

/// The structure that defines what action is required when finding regular expression matches.
/// ## MatchRequirement
/// - `MustBeFound` - which means we must necessarily get a match from this regular expression
/// - `MustNotBeFound` - which means, based on this regular expression, we must not get a match
///
/// Behavior of `Rule` | `RuleBytes` under different conditions
///
/// | **MatchRequirement** | **Match found** | *does this rule have any subrules ?* | Result                         |
/// | ---------------- | ----------- | ---------------------------------- | ---------------------------------------- |
/// | MustBeFound      | Yes         | Yes                                | Continue processing subrules             |
/// | MustBeFound      | Yes         | No                                 | Finish processing                        |
/// | MustBeFound      | No          | Yes                                | Error (match must have been found)       |
/// | MustBeFound      | No          | No                                 | Error (match must have been found)       |
/// | MustNotBeFound   | Yes         | Yes                                | Continue processing subrules             |
/// | MustNotBeFound   | Yes         | No                                 | Error (match should not have been found) |
/// | MustNotBeFound   | No          | Yes                                | Finish processing                        |
/// | MustNotBeFound   | No          | No                                 | Finish processing                        |
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
