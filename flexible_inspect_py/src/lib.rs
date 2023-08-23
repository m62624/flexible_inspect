//! The `Flexible_inspect` is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\
//! The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.

mod cartridges;
mod error;
mod rules;
mod template_validator;
#[cfg(test)]
mod unit_tests;
pub use cartridges::{cartridges_bytes::PyCartridgeBytes, cartridges_str::PyCartridge};
use flexible_inspect_rs::error_messages::*;
use flexible_inspect_rs::prelude::*;
use pyo3::exceptions;
use pyo3::prelude::*;
pub use rules::PyMatchRequeriment;
pub use rules::{rule_bytes::PyRuleBytes, rule_str::PyRule};
pub use template_validator::validate_bytes::PyTemplateValidatorBytes;
pub use template_validator::validate_str::PyTemplateValidator;

#[pyfunction]
pub fn init_logger_with_offset(hour_offset: i32) {
    flexible_inspect_rs::logs::init_logger_with_offset(hour_offset);
}

#[cfg(not(tarpaulin_include))]
#[pymodule]
pub fn flexible_inspect_py(_py: Python<'_>, py_module: &PyModule) -> PyResult<()> {
    py_module.add_class::<PyMatchRequeriment>()?;
    py_module.add_class::<PyRule>()?;
    py_module.add_class::<PyRuleBytes>()?;
    py_module.add_class::<PyCartridge>()?;
    py_module.add_class::<PyCartridgeBytes>()?;
    py_module.add_class::<PyTemplateValidator>()?;
    py_module.add_class::<PyTemplateValidatorBytes>()?;
    py_module.add_function(wrap_pyfunction!(init_logger_with_offset, py_module)?)?;
    Ok(())
}
