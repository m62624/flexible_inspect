mod context_match;
use super::*;
use crate::core::rules::next::NextStep;
use crate::core::rules::traits::{CalculateValueRules, RuleBase};
use log::debug;
use std::fmt::Display;
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
pub fn run<'a, R, C>(rule: &R::RuleType, data: C) -> NextStep
where
    C: PartialEq + Eq + Hash + Debug,
    R: CalculateValueRules<'a, C> + Debug,
{
    // ============================= LOG =============================
    debug!("running the root rule `{}`", rule.get_str());
    // ============================= LOG =============================

    let mut stack = VecDeque::from([(rule, R::find_captures(&rule, &data))]);
    while let Some(frame) = stack.front() {
        match frame.0.get_mode_match() {
            ModeMatch::AllRulesForAllMatches => {
                if let NextStep::Error(value) =
                    context_match::all_rules_for_all_matches::<R, C>(rule, &mut stack)
                {
                    return NextStep::Error(value);
                }
            }
            ModeMatch::AllRulesForAtLeastOneMatch => {
                // TODO: implement
                todo!()
            }
            ModeMatch::AtLeastOneRuleForAllMatches => {
                // TODO: implement
                todo!()
            }
            ModeMatch::AtLeastOneRuleForAtLeastOneMatch => {
                // TODO: implement
                todo!()
            }
        }
    }
    NextStep::Finish
}
