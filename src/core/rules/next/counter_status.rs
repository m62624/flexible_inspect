use crate::core::rules::Counter;

use super::*;

impl Counter {
    pub fn counter_status<T: RuleBase>(rule: T, captures: &mut CaptureData) -> NextStep {
        if let Some(counter) = rule.get_counter() {
            match counter {
                Counter::Only(value) => {
                    if captures.counter_value == value {
                        return NextStep::Finish;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                Counter::MoreThan(value) => {
                    if captures.counter_value >= value {
                        return NextStep::Finish;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                Counter::LessThan(value) => {
                    if captures.counter_value <= value {
                        return NextStep::Finish;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
            }
        }
        NextStep::Finish
    }
}
