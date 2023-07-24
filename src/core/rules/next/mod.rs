use super::{captures::CaptureData, traits::RuleBase};
use crate::MatchRequirement;
use std::collections::HashMap;

/// All optional modifiers return `NextStep` to have a unified type
#[derive(Debug)]
pub enum NextStep {
    Go,
    Finish,
    Error(Option<HashMap<String, String>>),
}

impl NextStep {
    pub fn next_or_finish_or_error<T: RuleBase>(rule: T, captures: &mut CaptureData) -> NextStep {
        match rule.get_requirement() {
            MatchRequirement::MustBeFound => {
                match (captures.is_some(), rule.get_subrules().is_some()) {
                    (true, true) => {
                        // TODO: check modifiers
                        return NextStep::Go;
                    }
                    (true, false) => {
                        // TODO: check modifiers
                        return NextStep::Finish;
                    }
                    (false, true) => {
                        // TODO: check modifiers
                        return NextStep::Error(None);
                    }
                    (false, false) => {
                        // TODO: check modifiers
                        return NextStep::Error(None);
                    }
                }
            }
            MatchRequirement::MustNotBeFound => {
                match (captures.is_some(), rule.get_subrules().is_some()) {
                    (true, true) => {
                        // TODO: check modifiers
                        return NextStep::Go;
                    }
                    (true, false) => {
                        // TODO: check modifiers
                        return NextStep::Error(Some(std::mem::take(
                            &mut captures.hashmap_for_error,
                        )));
                    }
                    (false, true) => {
                        // TODO: check modifiers
                        return NextStep::Finish;
                    }
                    (false, false) => {
                        // TODO: check modifiers
                        return NextStep::Finish;
                    }
                }
            }
        }
        NextStep::Finish
    }
}
