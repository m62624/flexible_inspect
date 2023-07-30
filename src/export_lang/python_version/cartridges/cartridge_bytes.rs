use crate::core::{cartridges::Cartridge, rules::rule_bytes::RuleBytes};
use crate::export_lang::python_version::rules::rule_bytes::PyRuleBytes;
use crate::export_lang::python_version::rules::traits::PyRuleModifiers;
use pyo3::prelude::*;
use pyo3::pymethods;

use super::traits::PyCartridgeBase;

#[pyclass(name = "CartridgeBytes")]
#[derive(Default, Clone, Debug)]
pub struct PyCartridgeBytes(Cartridge<RuleBytes>);

#[pymethods]
impl PyCartridgeBytes {
    #[new]
    pub fn new(py: Python, id: i64, message: String, root_rules: PyObject) -> PyResult<Self> {
        Ok(Self(Cartridge::new(
            id,
            message,
            PyRuleBytes::_to_rust_for_extend::<PyRuleBytes>(py, root_rules, "RuleBytes")?,
        )))
    }

    /// modifier to change the rule matching mode,
    /// `all rules` must pass the test for at least `one match`
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for `all matches`
    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for at least `one match`
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }
}

impl PyCartridgeBase for PyCartridgeBytes {
    type CartridgeType = Cartridge<RuleBytes>;
    fn to_rust(&mut self) -> Self::CartridgeType {
        std::mem::take(&mut self.0)
    }
}
