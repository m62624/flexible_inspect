// =======================================================
mod another_traits;
mod base;
mod captures;
mod init;
mod modifiers;
mod utils;
// ======================================================
use super::*;

/// A rule is the minimum unit of logic in a validator.
/// The rule supports two regular expression crates:
/// [**Regex**](https://crates.io/crates/regex) and [**FancyRegex**](https://crates.io/crates/fancy-regex).
/// Determines which type is used based on the syntax (for example, if *Lookahead* and *Lookbehind* references are used, this automatically defines as [**FancyRegex**](https://crates.io/crates/fancy-regex)).
///
/// The most important feature is that the rule is recursive (don't worry, recursion is not used here).
/// Each rule can have nested rules, and these nested rules can have their own nested rules, and so on.
/// Thus, when the root rule is triggered, all the results obtained are passed to the nested rules, so you can build complex structural rules to suit any taste
///
/// # Notes
/// * Remember any modifier takes the contents of the `Rule` body
/// and returns a new one with a changed parameter (only `None` from the original Rule remains),
/// so specify the modifier in the same place where you initialize `Rule`.
/// * If you stick with the [**Regex**](https://crates.io/crates/regex) library features, all root and nested rules go into [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html).
/// Many expressions can be accommodated in a regular expression without *Lookahead* and *Lookbehind* references.
/// But this is just a recommendation. If you need to use references, of course you can specify them.
/// Then these rules will not be included in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html),
/// and if there are rules in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html) they will be the first in the queue to be checked, and those that use [**FancyRegex**](https://crates.io/crates/fancy-regex) features will be at the end of the queue
/// * Basically use `Rule` instead of [`RuleBytes`](crate::prelude::RuleBytes) when working with text (not necessarily just text, it also includes `html` structures, code fragments from other languages, etc.) since it has support for [**Regex**](https://crates.io/crates/regex) and [**FancyRegex**](https://crates.io/crates/fancy-regex).
/// * How is recursive structure checking performed without recursion?
/// Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion

/*
The structure for checking strings with regular expressions.
Stores all values in the `Option`, so that if we change the modifiers we can return this structure again without `cloning`.
If we just implemented the method with `&mut self`,
we would change the internal values of the modifiers, but we would not return the structure itself.
Therefore, to avoid cloning the structure again, we borrow it via `mem::take`.
*/
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Rule(Option<TakeRuleForExtend>);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TakeRuleForExtend {
    pub str_with_type: RegexRaw,
    pub general_modifiers: GeneralModifiers,
    pub subrules: Option<Subrules>,
}

/// A structure for storing regular expressions
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexRaw {
    DefaultRegex(Box<str>),
    FancyRegex(Box<str>),
}

/// A structure that stores a set of regular expressions.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subrules {
    pub simple_rules: Option<SimpleRules>,
    pub complex_rules: Option<IndexSet<Rule>>,
}

/// Structure that stores regular expressions from which you can initialize in `RegexSet`
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
