// =======================================================
mod another_traits;
mod getters;
mod init;
mod modifiers;
use std::collections::HashSet;

// =======================================================
use super::*;

/// A structure for checking bytes with regular expressions

/*
Stores all values in the `Option`, so that if we change the modifiers we can return this structure again without `cloning`.
If we just implemented the method with `&mut self`,
we would change the internal values of the modifiers, but we would not return the structure itself.
Therefore, to avoid cloning the structure again, we borrow it via `mem::take`.
*/
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
    /// Here the queue can change, not the collection itself,
    /// but the way of processing rules at the beginning from `RegexSet`
    /// and then those that are not in `RegexSet` through `!contains`, so we use `IndexSet` for better performance.
    pub all_rules: IndexSet<RuleBytes>,
    /// `RegexSet` Match multiple, possibly overlapping, regexes in a single search.
    pub regex_set: regex::bytes::RegexSet,
}

pub struct CaptureDataBytes<'s> {
    pub text_for_capture: HashSet<&'s [u8]>,
    pub hashmap_for_error: HashMap<String, &'s [u8]>,
    pub counter_value: usize,
}
