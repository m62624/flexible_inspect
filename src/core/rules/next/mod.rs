mod counter_status;
mod modifier_arena;
use super::{traits::RuleBase, *};

/// All optional modifiers `NextStep` to have a unified type
#[derive(Debug, PartialEq, Eq)]
pub enum NextStep {
    Go,
    Finish,
    Error(Option<HashMap<String, String>>),
}

impl NextStep {
    pub fn next_or_finish_or_error<R: RuleBase, C: PartialEq + Eq + Hash>(
        rule: &R,
        captures: &mut CaptureData<C>,
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
