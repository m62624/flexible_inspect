use super::*;
use crate::core::rules::traits::RuleModifiers;
use crate::export_lang::python_version::rules::traits::PyRuleModifiers;
use pyo3::PyResult;

impl PyRuleModifiers for PyRule {
    type RuleTypePy = PyRule;
}

#[pymethods]
impl PyRule {
    pub fn extend(&mut self, py: Python, nested_rules: PyObject) -> PyResult<Self> {
        let collect = self.to_rust_for_extend::<PyRule>(py, nested_rules, "Rule")?;
        self.0 = self.0.extend(collect);
        Ok(std::mem::take(self))
    }
}
