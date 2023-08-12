// =======================================================
mod another_traits;
mod base;
mod captures;
mod init;
mod modifiers;
mod utils;
// =======================================================
use super::*;

/// A rule is the minimum unit of logic in a validator.\
/// **Notes:**
/// * Remember any modifier takes the contents of the `RuleBytes` body 
/// and returns a new one with a changed parameter (only `None` from the original Rule remains), 
/// so specify the modifier in the same place where you initialize `RuleBytes`.
/// * Use `&[u8]` when searching for regex matches in haystacks. ([**FancyRegex**](https://crates.io/crates/fancy-regex) capabilities are not available)
/// * Unicode support can be disabled, even if disabling it will result in a match with invalid `UTF-8` bytes
// More info at [link](https://docs.rs/regex/latest/regex/bytes/index.html) 
/*
The structure for checking bytes with regular expressions
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
