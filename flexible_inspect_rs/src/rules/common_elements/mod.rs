mod utils;

use self::range::RangeFormat;
use super::traits::{IntoSpecificCaptureType, RuleBase};
use super::*;
use std::marker::PhantomData;
pub mod range;
// =======================================================
/// This is reserved standard value for error filling
pub const DEFAULT_CAPTURE: &str = "main_capture";
// =======================================================

/// A structure for storing regular expressions
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexRaw {
    Standard(Box<str>),
    Fancy(Box<str>),
    Bytes(Box<str>),
}

/// The struct for sorting all nested rulesz
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SlisedRules<R: RuleBase> {
    /// The rules are in the `IndexSet` collection to preserve
    /// the order of the rules during index retrieval from the `RegexSet` and to avoid duplicate rules
    pub smr_must_be_found: IndexSet<R>,
    pub smr_must_not_be_found_with_subrules: IndexSet<R>,
    pub smr_must_not_be_found_without_subrules: IndexSet<R>,
    pub cmr: IndexSet<R>,
}

/// A Structure for common `Rule` modifiers
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralModifiers {
    pub requirement: MatchRequirement,
    pub counter: Option<Counter>,
    pub mode_match: ModeMatch,
    pub range: Option<RangeFormat>,
    pub save_duplicates: bool,
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

/// Data storage modes, mainly needed for `RuleBytes` when using `Range`
/// with byte reading modes `FromLeBytes` | `FromBeBytes` to store duplicates
#[derive(Debug)]
pub enum TypeStorageFormat<'a, T: IntoSpecificCaptureType<'a>> {
    /// A structure that stores a single value
    Single((IndexSet<T>, PhantomData<&'a T>)),
    /// A structure that stores multiple values
    Multiple((Vec<T>, PhantomData<&'a T>)),
}

/// A structure that stores all the data for processing the capture
#[derive(Debug)]
pub struct CaptureData<'a, T: IntoSpecificCaptureType<'a>> {
    pub text_for_capture: TypeStorageFormat<'a, T>,
    pub hashmap_for_error: HashMap<String, String>,
    pub counter_value: usize,
}
