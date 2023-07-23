// =======================================================
mod getters;
mod traits;
// =======================================================

use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RuleBytes(Option<TakeRuleBytesForExtend>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TakeRuleBytesForExtend {
    pub str_bytes: Box<str>,
    pub subrules_bytes: Option<SimpleRulesBytes>,
    pub general_modifiers: GeneralModifiers,
}

/// Structure that stores regular expressions from which you can initialize in `RegexSet`
#[derive(Debug, Clone)]
pub struct SimpleRulesBytes {
    pub all_rules: IndexSet<RuleBytes>,
    /// `RegexSet` Match multiple, possibly overlapping, regexes in a single search.
    pub regex_set: regex::RegexSet,
}
