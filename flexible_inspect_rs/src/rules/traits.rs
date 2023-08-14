/*
Here we implement traits for two types of `Rule`, they are string `Rule` and byte `Rule`.
They are necessary to avoid code duplicates. Especially in context_match, where there are several modes
*/

// =======================================================
use super::{CaptureData, Counter, ModeMatch};
use crate::prelude::MatchRequirement;
use indexmap::IndexSet;
use std::{fmt::Debug, hash::Hash};
// =======================================================

/// This trait requires implementations of the most basic methods for any `Rule`.
pub trait RuleBase {
    type TakeRuleType;
    type SubRulesType;
    type RuleType: Debug;
    type RegexSet;

    fn _new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self;
    fn content_unchecked(&self) -> &Self::TakeRuleType;
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType;
    fn get_subrules(&self) -> Option<&Self::SubRulesType>;
    fn get_simple_rules(&self) -> Option<(&IndexSet<Self::RuleType>, &Self::RegexSet)>;
    fn get_complex_rules(&self) -> Option<&IndexSet<Self::RuleType>>;
    fn get_requirement(&self) -> &MatchRequirement;
    fn get_counter(&self) -> Option<Counter>;
    fn get_mode_match(&self) -> &ModeMatch;
    fn get_str(&self) -> &str;
}

/// The main trait for `context_match`, that is,
/// the implementation of the modifier nesting logic will be common for two different rule structures.
/// That is, `next` + `mode matching` will be common for them.
/// The main thing is to implement separately `Captures` for `&str` and `&[u8]`
/// the rest will be the same

pub trait CalculateValueRules<'a, C: PartialEq + Eq + Hash> {
    type RuleType: RuleBase<RuleType = Self::RuleType, RegexSet = Self::RegexSet>
        + Hash
        + Eq
        + PartialEq;
    type RegexSet: 'a;
    fn get_selected_rules(regex_set: &Self::RegexSet, text: &C) -> Vec<usize>;
    fn find_captures(rule: &Self::RuleType, capture: &C) -> CaptureData<C>;
}

/// This trait requires modifier implementations for any `Rules`
pub trait RuleModifiers {
    /// The type of the rule that will be returned after applying the modifier
    type RuleType;

    /// modifier for extending the rule with nested rules
    /// 
    /// ( **by default, all rules must pass every match check** )\
    /// In this mode, to which all additional rules apply (default mode for everyone).
    /// We check that for each match (text) all the rules will work.
    /// ## Operation scheme of the mode
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///      |   [123], [456], [789]
    ///      |___ Subrule ".+" (MustBeFound) ---> [123] -> [456] -> [789] -- TRUE
    ///      |                                      |       |        |
    ///      |___ Subrule "\[\d+\]" (MustBeFound) __|_______|________|
    ///
    /// ```
    fn extend<R: IntoIterator<Item = Self::RuleType>>(&mut self, nested_rules: R)
        -> Self::RuleType;
    /// modifier to set the match counter, condition `counter == match`
    fn counter_is_equal(&mut self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter >= match`
    fn counter_more_than(&mut self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter <= match`
    fn counter_less_than(&mut self, count: usize) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, `all the sub-rule` should work for at least `one match`.
    /// If at least one sub-rule does not work on one of the matches, an error will be returned.
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError (Cartridge)
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///     |   [123], [456], [789]
    ///     |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE
    ///     |                                      |
    ///     |___ Subrule "\[\d+\]" (MustBeFound) __|
    ///     |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- ERROR
    /// ```
    fn all_r_for_any_m(&mut self) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least `one sub-rule` should work for `every match`. If no sub-rule works on one of the matches, an error will be returned.
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError (Cartridge)
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///     |   [123], [456], [789]
    ///     |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE -- [456] -- TRUE -- [789] -- TRUE
    ///     |                                      |               |                 |
    ///     |___ Subrule "\[\d+\]" (MustBeFound) __|_______________|_________________|
    ///     |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- TRUE (since other rules matched)
    /// ```
    fn any_r_for_all_m(&mut self) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least `one sub-rule` should work for at least `one match`. If no sub-rule works on one of the matches, an error will be returned.
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError (Cartridge)
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///     |   [123], [456], [789]
    ///     |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE
    ///     |                                      |
    ///     |___ Subrule "\[\d+\]" (MustBeFound) __|
    ///     |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- TRUE (since other rules matched for at least one match)
    /// ```
    fn any_r_for_any_m(&mut self) -> Self::RuleType;
}
