use super::*;
use crate::core::rules::Counter;

/// Additional checks of modifiers are performed here
/// ( if there will be more of them and they will take a long time to calculate, 
/// we will switch to checking via `async_task` for each modifier )
pub fn modifier_runner<T: RuleBase>(rule: T, captures: &mut CaptureData) -> NextStep {
    // ====================== COUNTER ======================
    if let NextStep::Error(value) = Counter::counter_status(rule, captures) {
        return NextStep::Error(value);
    }
    // ====================================================

    NextStep::Finish
}
