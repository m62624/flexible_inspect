// =======================================================
mod another_traits;
mod base;
mod captures;
mod init;
mod modifiers;
mod utils;
// =======================================================
use super::*;

/// The structure for checking bytes with regular expressions

/*
Stores all values in the `Option`, so that if we change the modifiers we can return this structure again without `cloning`.
If we just implemented the method with `&mut self`,
we would change the internal values of the modifiers, but we would not return the structure itself.
Therefore, to avoid cloning the structure again, we borrow it via `mem::take`.
*/
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RuleBytes(Option<TakeRuleBytesForExtend>);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TakeRuleBytesForExtend {
    pub str_bytes: Box<str>,
    pub general_modifiers: GeneralModifiers,
    pub subrules_bytes: Option<SimpleRulesBytes>,
}

/// Structure that stores regular expressions from which you can initialize in `RegexSet`
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct SimpleRulesBytes {
    /// The rules are in the `IndexSet` collection to preserve
    /// the order of the rules during index retrieval from the `RegexSet` and to avoid duplicate rules
    pub all_rules: IndexSet<RuleBytes>,
    /// `RegexSet` Match multiple, possibly overlapping, regexes in a single search.
    pub regex_set: RegexSetContainer,
}

#[derive(Debug, Clone)]
pub struct RegexSetContainer {
    pub regex_set: regex::bytes::RegexSet,
}
