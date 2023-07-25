use crate::core::rules::next::NextStep;
use crate::core::rules::traits::{CalculateValueRules, RuleBase};
use crate::core::rules::CaptureData;
use log::{debug, error, info};
use std::{collections::VecDeque, fmt::Debug, hash::Hash};

pub fn all_rules_for_all_matches<R, C>(
    rule_ref: &R::RuleType,
    stack: &mut VecDeque<(&R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    C: PartialEq + Eq + Hash + Debug,
    R: CalculateValueRules<C>,
    R::RuleType: RuleBase,
{
    // ============================= LOG =============================
    debug!("the local rule stack `{}` is received", {
        rule_ref.as_str()
    });
    // ============================= LOG =============================
    let mut temp_stack: VecDeque<(&R::RuleType, CaptureData<C>)> = VecDeque::new();
    while let Some(mut frame) = stack.pop_front() {
        // ============================= LOG =============================
        debug!(
            "\ncheck the state of the rule `{}` \nfrom the local stack `{}`",
            frame.0.as_str(),
            rule_ref.as_str()
        );
        // ============================= LOG =============================

        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                // ============================= LOG =============================
                debug!(
                    "success, run subrules from the root rule `{}`",
                    rule_ref.as_str()
                );
                // ============================= LOG =============================
            }
            NextStep::Finish => {
                // ============================= LOG =============================
                debug!(
                    "the rule `{}` is finished, the result is `Ok`",
                    rule_ref.as_str()
                );
                // ============================= LOG =============================
            }
            NextStep::Error(_) => {}
        }
    }
    NextStep::Finish
}
