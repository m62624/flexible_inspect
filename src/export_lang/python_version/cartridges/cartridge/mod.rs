use crate::core::{cartridges::Cartridge, rules::rule_str::Rule};
use crate::export_lang::python_version::rules::rule_str::PyRule;
use crate::export_lang::python_version::rules::traits::{PyRuleBase, PyRuleModifiers};
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods};

use super::traits::PyCartridgeBase;
#[pyclass(name = "Cartridge")]
#[derive(Default, Clone, Debug)]
pub struct PyCartridge(Cartridge<Rule>);

#[pymethods]
impl PyCartridge {
    #[new]
    pub fn new(py: Python, id: i64, message: String, root_rules: PyObject) -> PyResult<Self> {
        Ok(Self(Cartridge::new(
            id,
            message,
            PyRule::_to_rust_for_extend::<PyRule>(py, root_rules, "Rule")?,
        )))
    }
}

impl PyCartridgeBase for PyCartridge {
    type CartridgeType = Cartridge<Rule>;
    fn to_rust(&mut self) -> Self::CartridgeType {
        std::mem::take(&mut self.0)
    }
}
