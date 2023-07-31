pub mod cartridge_bytes;
pub mod cartridge_str;

use crate::core::rules::rule_str::Rule;
use crate::core::{cartridges::Cartridge, rules::rule_bytes::RuleBytes};
use crate::export_lang::python_version::rules::rule_bytes::PyRuleBytes;
use crate::export_lang::python_version::rules::rule_str::PyRule;
use crate::export_lang::python_version::rules::PyRuleModifiers;
use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::pymethods;
