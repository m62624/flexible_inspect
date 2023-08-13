mod context_match;
use super::*;
use crate::rules::next::NextStep;
use crate::rules::traits::{CalculateValueRules, RuleBase};
use colored::*;
use log::debug;
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
pub fn run<'a, R, C>(rule: &'a R::RuleType, data: CaptureData<C>) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: PartialEq + Eq + Hash + Debug,
{
    // ============================= LOG =============================
    debug!(
        "running the root rule `({}, {})`",
        rule.get_str().yellow(),
        format!("{:#?}", rule.get_requirement()).yellow()
    );
    // ===============================================================

    let mut stack = VecDeque::from([(rule, data)]);
    while let Some(frame) = stack.front() {
        debug!(
            ":: going through the `({}, {})` rule stack with `{}` mode",
            frame.0.get_str().yellow(),
            format!("{:#?}", frame.0.get_requirement()).yellow(),
            format!("{:#?}", frame.0.get_mode_match()).bright_yellow()
        );
        match frame.0.get_mode_match() {
            ModeMatch::AllRulesForAllMatches => {
                if let NextStep::Error(value) =
                    context_match::all_rules_for_all_matches::<R, C>(&mut stack)
                {
                    return NextStep::Error(value);
                }
            }
            ModeMatch::AllRulesForAtLeastOneMatch => {
                if let NextStep::Error(value) =
                    context_match::all_rules_for_at_least_one_match::<R, C>(&mut stack)
                {
                    return NextStep::Error(value);
                }
            }
            ModeMatch::AtLeastOneRuleForAllMatches => {
                if let NextStep::Error(value) =
                    context_match::at_least_one_rule_for_all_matches::<R, C>(&mut stack)
                {
                    return NextStep::Error(value);
                }
            }
            ModeMatch::AtLeastOneRuleForAtLeastOneMatch => {
                if let NextStep::Error(value) =
                    context_match::at_least_one_rule_for_at_least_one_match::<R, C>(&mut stack)
                {
                    return NextStep::Error(value);
                }
            }
        }
    }
    NextStep::Finish
}
