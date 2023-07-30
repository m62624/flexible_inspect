use crate::core::{cartridges::Cartridge, rules::rule_str::Rule};
use crate::export_lang::python_version::rules::rule_str::PyRule;
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods};

use super::traits::PyCartridgeBase;
#[pyclass(name = "Cartridge")]
#[derive(Default, Clone, Debug)]
pub struct PyCartridge(Cartridge<Rule>);

#[pymethods]
impl PyCartridge {
    #[new]
    pub fn new(root_rules: PyObject) -> Self {
        todo!()
    }
}

impl PyCartridgeBase for PyCartridge {
    type CartridgeType = Cartridge<Rule>;
    type RuleType = PyRule;
    fn to_rust(&mut self) -> Self::CartridgeType {
        std::mem::take(&mut self.0)
    }
}
