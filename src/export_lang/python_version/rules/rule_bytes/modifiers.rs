use super::*;
use crate::core::rules::traits::RuleModifiers;
use crate::export_lang::python_version::rules::traits::PyRuleModifiers;
use pyo3::PyResult;

impl PyRuleModifiers for PyRuleBytes {
    type RuleTypePy = PyRuleBytes;
}

#[pymethods]
impl PyRuleBytes {
    pub fn extend(&mut self, py: Python, nested_rules: PyObject) -> PyResult<Self> {
        let collect = self.to_rust_for_extend::<PyRuleBytes>(py, nested_rules, "RuleBytes")?;
        self.0 = self.0.extend(collect);
        Ok(std::mem::take(self))
    }
}
