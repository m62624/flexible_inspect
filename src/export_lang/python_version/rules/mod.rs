pub mod rule_bytes;
pub mod rule_str;
pub mod traits;

use crate::core::rules::rule_bytes::RuleBytes;
use crate::core::rules::rule_str::Rule;
use crate::core::rules::MatchRequirement;
use pyo3::prelude::*;
use pyo3::types;
use pyo3::{pyclass, pymethods};

#[pyclass(name = "MatchRequirement")]
#[derive(Clone)]
pub enum PyMatchRequirement {
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
