// =======================================================
mod getters;
mod init;
mod traits;
// =======================================================

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
    pub complex_rules: Option<Vec<Rule>>,
    pub bytes_rules: Option<IndexSet<Rule>>,
}

/// Structure that stores regular expressions from which you can initialize in `RegexSet`
#[derive(Debug, Clone)]
pub struct SimpleRules {
    pub all_rules: IndexSet<Rule>,
    /// `RegexSet` Match multiple, possibly overlapping, regexes in a single search.
    pub regex_set: regex::RegexSet,
}
