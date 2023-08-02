// =======================================================
mod another_traits;
mod captures;
mod base;
mod init;
mod modifiers;
mod utils;
// ======================================================
use super::*;

/// The structure for checking strings with regular expressions

/*
Stores all values in the `Option`, so that if we change the modifiers we can return this structure again without `cloning`.
If we just implemented the method with `&mut self`,
we would change the internal values of the modifiers, but we would not return the structure itself.
Therefore, to avoid cloning the structure again, we borrow it via `mem::take`.
*/
#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Rule(Option<TakeRuleForExtend>);

#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TakeRuleForExtend {
    pub str_with_type: RegexRaw,
    pub general_modifiers: GeneralModifiers,
    pub subrules: Option<Subrules>,
}

/// A structure for storing regular expressions
#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexRaw {
    DefaultRegex(Box<str>),
    FancyRegex(Box<str>),
}

/// A structure that stores a set of regular expressions.
#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subrules {
    pub simple_rules: Option<SimpleRules>,
    pub complex_rules: Option<IndexSet<Rule>>,
}

/// Structure that stores regular expressions from which you can initialize in `RegexSet`
#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone)]
pub struct SimpleRules {
    /// The rules are in the `IndexSet` collection to preserve
    /// the order of the rules during index retrieval from the `RegexSet` and to avoid duplicate rules
    pub all_rules: IndexSet<Rule>,
    /// `RegexSet` Match multiple, possibly overlapping, regexes in a single search.
    pub regex_set: RegexSetContainer,
}

#[derive(Debug, Clone)]
pub struct RegexSetContainer {
    pub regex_set: regex::RegexSet,
}