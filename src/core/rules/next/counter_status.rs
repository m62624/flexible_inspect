use super::*;
use crate::core::rules::{
    traits::{CalculateValueRules, RuleBase},
    Counter,
};

impl Counter {
    pub fn counter_status<R: RuleBase, T: PartialEq + Eq + Hash>(
        rule: R,
        captures: &mut CaptureData<T>,
    ) -> NextStep {
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
