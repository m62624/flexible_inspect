use super::captures::CaptureData;
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum NextStep {
    Go,
    Finish,
    Error(Option<HashMap<String, String>>),
}

impl Rule {
    pub fn next_or_data_for_error<'a>(rule: &rule::Rule, captures: &mut CaptureData) -> NextStep {
        match rule.content_unchecked().requirement {
            MatchRequirement::MustBeFound => {
                match (
                    captures.is_some(),
                    rule.content_unchecked().subrules.is_some(),
                ) {
                    (true, true) => rule.counter_status(captures),
                    (true, false) => NextStep::Finish,
                    (false, true) => {
                        NextStep::Error(None)
                    }
                    (false, false) => {
                        NextStep::Error(None)
                    }
                }
            }
            MatchRequirement::MustNotBefound => {
                match (
                    captures.is_some(),
                    rule.content_unchecked().subrules.is_some(),
                ) {
                    (true, true) => rule.counter_status(captures),
                    (true, false) => {
                        NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)))
                    }
                    (false, true) => NextStep::Finish,
                    (false, false) => NextStep::Finish,
                }
            }
        }
    }

    fn counter_status(&self, captures: &mut CaptureData) -> NextStep {
        if let Some(value) = self.content_unchecked().counter {
            match value {
                Counter::Only(value) => {
                    if captures.counter_value == value {
                        return NextStep::Go;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                Counter::MoreThan(value) => {
                    if captures.counter_value >= value {
                        return NextStep::Go;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                Counter::LessThan(value) => {
                    if captures.counter_value <= value {
                        return NextStep::Go;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
            }
        }
        NextStep::Go
    }
}
