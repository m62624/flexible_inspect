use std::collections::HashMap;

/// All optional modifiers return `NextStep` to have a unified type
#[derive(Debug, PartialEq, Eq)]
pub enum NextStep {
    Go,
    Finish,
    Error(Option<HashMap<String, String>>),
}
