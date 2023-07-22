use super::*;
use indexmap::IndexSet;
use std::hash::Hash;
use std::mem;

// =====================================================================
mod getters;
mod init;
mod modifiers;
mod slice;
mod traits;
// =====================================================================

/// The basic unit of the validator, it stores a regular expression with modifiers
/*
Stores all values in the `Option`, so that if we change the modifiers we can return this structure again without `cloning`.
If we just implemented the method with `&mut self`,
we would change the internal values of the modifiers, but we would not return the structure itself.
Therefore, to avoid cloning the structure again, we borrow it via `mem::take`.
*/
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Rule(Option<TakeRuleForExtend>);

/// We divide using the structure into a hidden modifier, which cannot be changed, and what can be changed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TakeRuleForExtend {
    pub str_with_type: RegexRaw,
    pub mutable_modifiers: MutableModifiers,
}

/// Separating types of regular expressions. The hidden modifier that is defined based on the regular expression. Only RegexBytes are excluded. This can be specified if necessary.
/*
A structure for storing the strings themselves, wrapped in a Box,
so there is a lifetime limit if the structure is exported to other languages.

*/
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexRaw {
    DefaultRegex(Box<str>),
    FancyRegex(Box<str>),
    RegexBytes(Box<str>),
}

/// A structure for storing all modifiers that can be changed by the user.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutableModifiers {
    pub requirement: MatchRequirement,
    pub subrules: Option<Subrules>,
    pub counter: Option<Counter>,
    pub mod_match: ModeMatch,
}

/// A structure that defines what action is required when finding regular expression matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBeFound,
}

/// A structure that stores a set of regular expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subrules {
    pub simple_rules: Option<SimpleRules>,
    pub complex_rules: Option<Vec<Rule>>,
    pub bytes_rules: Option<IndexSet<Rule>>,
}

/// The struct for sorting all nested rules
pub struct SlisedRules {
    /// `IndexSet` provides access to items in `O(1)` time on average when using the contains method.
    pub simple_rules: IndexSet<Rule>,
    pub complex_rules: Vec<Rule>,
    /// `IndexSet` provides access to items in `O(1)` time on average when using the contains method.
    pub bytes_rules: IndexSet<Rule>,
}

/// Structure that stores regular expressions from which you can initialize in `RegexSet`
#[derive(Debug, Clone)]
pub struct SimpleRules {
    /// `IndexSet` provides access to items in `O(1)` time on average when using the contains method.
    pub all_rules: IndexSet<Rule>,
    /// `RegexSet` Match multiple, possibly overlapping, regexes in a single search.
    pub regex_set: regex::RegexSet,
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
