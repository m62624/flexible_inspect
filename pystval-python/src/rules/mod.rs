mod rule_bytes;
mod rule_str;
use super::*;

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
