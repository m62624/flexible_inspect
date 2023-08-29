use super::number_range_status::number_range_status;
use super::rules::{traits::RuleBase, Counter};
use super::*;

/// Additional checks of modifiers are performed here
/// ( if there will be more of them and they will take a long time to calculate,
/// we will switch to checking via `async_task` for each modifier )
pub fn modifier_runner<'a, R: RuleBase, C: IntoConcreteType<'a>>(
    rule: &R,
    captures: &mut CaptureData<'a, C>,
) -> NextStep {
    // ====================== COUNTER ======================
    if let NextStep::Error(value) = Counter::counter_status(rule, captures) {
        return NextStep::Error(value);
    }
    if let NextStep::Error(value) = number_range_status(rule, captures) {
        return NextStep::Error(value);
    }
    // ====================================================

    NextStep::Finish
}
