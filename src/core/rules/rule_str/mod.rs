// =======================================================
mod another_traits;
mod captures;
mod getters;
mod init;
mod modifiers;
mod utils;
// ======================================================
use super::*;

/// A structure for checking strings with regular expressions

/*
Stores all values in the `Option`, so that if we change the modifiers we can return this structure again without `cloning`.
If we just implemented the method with `&mut self`,
we would change the internal values of the modifiers, but we would not return the structure itself.
Therefore, to avoid cloning the structure again, we borrow it via `mem::take`.
*/
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Rule(Option<TakeRuleForExtend>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TakeRuleForExtend {
    pub str_with_type: RegexRaw,
    pub subrules: Option<Subrules>,
    pub general_modifiers: GeneralModifiers,
}

/// A structure for storing regular expressions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexRaw {
    DefaultRegex(Box<str>),
    FancyRegex(Box<str>),
}

/// A structure that stores a set of regular expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subrules {
    pub simple_rules: Option<SimpleRules>,
    /// Since we are looping through all elements without changing the queue order, we can use the vector
    pub complex_rules: Option<Vec<Rule>>,
}

/// Structure that stores regular expressions from which you can initialize in `RegexSet`
#[derive(Debug, Clone)]
pub struct SimpleRules {
    /// Here the queue can change, not the collection itself,
    /// but the way of processing rules at the beginning from `RegexSet`
    /// and then those that are not in `RegexSet` through `!contains`, so we use `IndexSet` for better performance.
    pub all_rules: IndexSet<Rule>,
    /// `RegexSet` Match multiple, possibly overlapping, regexes in a single search.
    pub regex_set: regex::RegexSet,
}
