use super::{captures::CaptureData, traits::RuleBase};
use crate::MatchRequirement;
use std::collections::HashMap;

/// All optional modifiers return `NextStep` to have a unified type
#[derive(Debug, PartialEq, Eq)]
pub enum NextStep {
    Go,
    Finish,
    Error(Option<HashMap<String, String>>),
}

impl NextStep {
    pub fn next_or_finish_or_error<T: RuleBase>(rule: T, captures: CaptureData) -> NextStep {
        match rule.get_requirement() {
            MatchRequirement::MustBeFound => {
                match (captures.is_some(), rule.get_subrules().is_some()) {
                    (true, true) => todo!(),
                    (true, false) => todo!(),
                    (false, true) => todo!(),
                    (false, false) => todo!(),
                }
            }
            MatchRequirement::MustNotBeFound => {
                match (captures.is_some(), rule.get_subrules().is_some()) {
                    (true, true) => todo!(),
                    (true, false) => todo!(),
                    (false, true) => todo!(),
                    (false, false) => todo!(),
                }
            }
        }
        todo!()
    }
}
