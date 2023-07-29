mod rule_bytes;
mod rule_str;

use crate::core::rules::rule_bytes::RuleBytes;
use crate::core::rules::rule_str::Rule;
use crate::core::rules::MatchRequirement;
use pyo3::{pyclass, pymethods};

#[pyclass(name = "MatchRequirement")]
#[derive(Clone)]
enum PyMatchRequirement {
    MustBeFound,
    MustNotBeFound,
}

impl PyMatchRequirement {
    pub fn to_rust(&self) -> MatchRequirement {
        match self {
            PyMatchRequirement::MustBeFound => MatchRequirement::MustBeFound,
            PyMatchRequirement::MustNotBeFound => MatchRequirement::MustNotBeFound,
        }
    }
}
