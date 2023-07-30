use crate::core::{cartridges::Cartridge, rules::rule_bytes::RuleBytes};
use pyo3::pyclass;

#[pyclass(name = "CartridgeBytes")]
#[derive(Default, Clone, Debug)]
pub struct PyCartridgeBytes(Cartridge<RuleBytes>);
