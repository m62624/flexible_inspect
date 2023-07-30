mod tests_cartridges_bytes;
mod tests_cartridges_str;

use crate::core::rules::traits::RuleBase;
use crate::core::rules::ModeMatch;
use crate::export_lang::python_version::cartridges::cartridge_bytes::PyCartridgeBytes;
use crate::export_lang::python_version::cartridges::cartridge_str::PyCartridge;
use crate::export_lang::python_version::cartridges::traits::PyCartridgeBase;
use crate::export_lang::python_version::rules::rule_bytes::PyRuleBytes;
use crate::export_lang::python_version::rules::rule_str::PyRule;
use crate::export_lang::python_version::unit_tests::FakeObject;
use pyo3::{types::PyList, IntoPy, Python};
