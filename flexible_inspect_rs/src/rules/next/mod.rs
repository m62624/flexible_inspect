mod counter_status;
mod modifier_arena;
pub mod number_range_status;
use super::{
    traits::{IntoSpecificCaptureType, RuleBase},
    *,
};

/// All optional modifiers `NextStep` to have a unified type
#[derive(Debug, PartialEq, Eq)]
pub enum NextStep {
    /// Continue to the next `Rule|RuleBytes`
    Go,
    /// Finish the step
    Finish,
    /// Error with optional `HashMap` for error filling
    Error(Option<HashMap<String, String>>),
}

impl NextStep {
    /// Mechanism with final variant, depending on modifiers we get the result
    pub fn next_or_finish_or_error<'a, R: RuleBase, C: IntoSpecificCaptureType<'a>>(
        rule: &R,
        captures: &mut CaptureData<'a, C>,
    ) -> NextStep {
        match rule.get_requirement() {
            MatchRequirement::MustBeFound => {
                match (captures.is_some(), rule.get_subrules().is_some()) {
                    (true, true) => {
                        if let NextStep::Error(value) =
                            modifier_arena::modifier_runner(rule, captures)
                        {
                            return NextStep::Error(value);
                        };
                        NextStep::Go
                    }
                    (true, false) => {
                        if let NextStep::Error(value) =
                            modifier_arena::modifier_runner(rule, captures)
                        {
                            return NextStep::Error(value);
                        };
                        NextStep::Finish
                    }
                    (false, true) => NextStep::Error(None),
                    (false, false) => NextStep::Error(None),
                }
            }
            MatchRequirement::MustNotBeFound => {
                match (captures.is_some(), rule.get_subrules().is_some()) {
                    (true, true) => {
                        if let NextStep::Error(value) =
                            modifier_arena::modifier_runner(rule, captures)
                        {
                            return NextStep::Error(value);
                        };
                        NextStep::Go
                    }
                    (true, false) => {
                        NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)))
                    }
                    (false, true) => {
                        if let NextStep::Error(value) =
                            modifier_arena::modifier_runner(rule, captures)
                        {
                            return NextStep::Error(value);
                        };
                        NextStep::Finish
                    }
                    (false, false) => {
                        if let NextStep::Error(value) =
                            modifier_arena::modifier_runner(rule, captures)
                        {
                            return NextStep::Error(value);
                        };
                        NextStep::Finish
                    }
                }
            }
        }
    }
}
