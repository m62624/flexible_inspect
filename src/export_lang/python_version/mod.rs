mod rules;

use pyo3::{pymodule, types::PyModule, PyResult, Python};

#[pymodule]
pub fn pystval(_py: Python<'_>, py_module: &PyModule) -> PyResult<()> {
    py_module.add_class::<rules::rule_str::PyRule>()?;
    py_module.add_class::<rules::rule_bytes::PyRuleBytes>()?;
    py_module.add_class::<rules::PyMatchRequirement>()?;
    Ok(())
}
