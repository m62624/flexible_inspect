use super::*;
use crate::core::rules::{
    next::NextStep,
    traits::{CalculateValueRules, RuleBase},
};
use std::{collections::VecDeque, fmt::Debug};

/// Main method for iteratively running a rule
/// Goes through all subrules, and so on to the end for each `Rule`.
///
/*
Step 1
 For each rule, the first step is to check the `RegexSet`, our task is to
 to return a validation error if any rule fails.
 So, it's in our best interest to immediately check what's already found
 via `RegexSet`
 In this way, we try to get the results of the simplest rules as quickly as possible.
 (by simple, we mean rules that are immediately included in the `RegexSet`)

Step 2
 Next are those rules that are not included in `RegexSet`, but that's how it should be.
 Because `RegexSet` selects those that are found and only then their modifiers are checked.
 And those not selected may have the same modifiers `MustBeFound` and MustNotBefound. If a pattern is not found and `MustBeFound` is specified, it is an error, so we check them in the second step.

Step 3
 Next we check `FancyRegex` which has lookaround & backreferences, such rules may take longer to process.
 Of course, it depends on the pattern itself, but in general, they can be longer than regular rules.
 That's why we leave them at the end to try to weed out long calculations at the beginning of the queue
*/
pub fn run<R: CalculateValueRules<T>, T>(rule: &R::RuleType, data: T) -> NextStep
where
    T: PartialEq + Eq + Hash + Debug,
    R::RuleType: RuleBase,
{
    let mut stack: VecDeque<(&<R as CalculateValueRules<T>>::RuleType, CaptureData<T>)> =
        VecDeque::from([(rule, R::find_captures(&rule, data))]);
    while let Some(frame) = stack.front() {
        frame.0.content_unchecked();
    }
    NextStep::Finish
}
