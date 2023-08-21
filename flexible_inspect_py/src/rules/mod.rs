pub mod rule_bytes;
pub mod rule_str;
use super::*;

const ERR_OPTION: &str = "\nThe body of `Rule` is missing (inside Rule is the value None), you may have used modifiers separately from initializations, they take the value (ownership) of `Rule` (std::mem::take) and return the already modified version (specify the modifier in the same place where you initialize `Rule|RuleBytes`).\n";

#[pyclass(name = "MatchRequirement")]
#[derive(Clone)]
pub enum PyMatchRequeriment {
    MustBeFound,
    MustNotBeFound,
}

impl From<PyMatchRequeriment> for MatchRequirement {
    fn from(value: PyMatchRequeriment) -> Self {
        match value {
            PyMatchRequeriment::MustBeFound => MatchRequirement::MustBeFound,
            PyMatchRequeriment::MustNotBeFound => MatchRequirement::MustNotBeFound,
        }
    }
}
