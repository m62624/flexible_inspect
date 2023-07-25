mod counter_status;
use super::*;

/// All optional modifiers `NextStep` to have a unified type
#[derive(Debug, PartialEq, Eq)]
pub enum NextStep {
    Go,
    Finish,
    Error(Option<HashMap<String, String>>),
}
