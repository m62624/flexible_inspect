mod cartridges;
mod rules;
#[cfg(test)]
mod unit_tests;
mod validator;

use pyo3::{pymodule, types::PyModule, PyResult, Python};

#[cfg(not(tarpaulin_include))]
#[pymodule]
pub fn pystval(_py: Python<'_>, py_module: &PyModule) -> PyResult<()> {
    py_module.add_class::<cartridges::cartridge_str::PyCartridge>()?;
    py_module.add_class::<cartridges::cartridge_bytes::PyCartridgeBytes>()?;
    py_module.add_class::<rules::rule_str::PyRule>()?;
    py_module.add_class::<rules::rule_bytes::PyRuleBytes>()?;
    py_module.add_class::<rules::PyMatchRequirement>()?;
    Ok(())
}
