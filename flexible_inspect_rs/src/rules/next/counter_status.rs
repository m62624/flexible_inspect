use log::error;

use super::*;
use crate::rules::{traits::RuleBase, Counter};

/// The counter implementation
impl Counter {
    pub fn counter_status<R: RuleBase, C: PartialEq + Eq + Hash>(
        rule: &R,
        captures: &mut CaptureData<C>,
    ) -> NextStep {
        if let Some(counter) = rule.get_counter() {
            match counter {
                Counter::Only(value) => {
                    if captures.counter_value == value {
                        return NextStep::Finish;
                    }
                    error!(
                        "the counter value `{}` is not equal to the required value `({:?})`",
                        captures.counter_value,
                        rule.get_counter(),
                    );
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                Counter::MoreThan(value) => {
                    if captures.counter_value >= value {
                        return NextStep::Finish;
                    }
                    error!(
                        "the counter value `{}` is not equal to the required value `({:?})`",
                        captures.counter_value,
                        rule.get_counter(),
                    );
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                Counter::LessThan(value) => {
                    if captures.counter_value <= value {
                        return NextStep::Finish;
                    }
                    error!(
                        "the counter value `{}` is not equal to the required value `({:?})`",
                        captures.counter_value,
                        rule.get_counter(),
                    );
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
            }
        }
        NextStep::Finish
    }
}
