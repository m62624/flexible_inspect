pub mod rule_bytes;
pub mod rule_str;
use crate::core::rules::rule_bytes::RuleBytes;
use crate::core::rules::rule_str::Rule;
use crate::core::rules::MatchRequirement;
use log::error;
use pyo3::prelude::*;
use pyo3::types;
use pyo3::{exceptions, FromPyObject, Python};
use pyo3::{pyclass, pymethods};

#[pyclass(name = "MatchRequirement")]
#[derive(Clone)]
pub enum PyMatchRequirement {
    MustBeFound,
    MustNotBeFound,
}

impl From<PyMatchRequirement> for MatchRequirement {
    fn from(value: PyMatchRequirement) -> Self {
        match value {
            PyMatchRequirement::MustBeFound => MatchRequirement::MustBeFound,
            PyMatchRequirement::MustNotBeFound => MatchRequirement::MustNotBeFound,
        }
    }
}

pub trait PyRuleModifiers {
    type PyRuleType;
    type RustRuleType;
    /// modifier to set the match counter, condition counter == match
    fn _counter_is_equal(&mut self, count: usize) -> Self::PyRuleType;
    fn _counter_more_than(&mut self, count: usize) -> Self::PyRuleType;

    fn _counter_less_than(&mut self, count: usize) -> Self::PyRuleType;
    fn _mode_all_rules_for_at_least_one_match(&mut self) -> Self::PyRuleType;
    fn _mode_at_least_one_rule_for_all_matches(&mut self) -> Self::PyRuleType;
    fn _mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::PyRuleType;
    fn _to_rust_for_extend<ConvertToRust>(
        py: Python,
        nested_rules: PyObject,
        message_type_rule: &str,
    ) -> PyResult<Vec<Self::RustRuleType>>
    where
        ConvertToRust: for<'a> FromPyObject<'a> + Into<Self::RustRuleType>,
    {
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            Ok(list
                .iter()
                .map(|py_rule| {
                    if let Ok(rule) = py_rule.extract::<ConvertToRust>() {
                        Ok(rule.into())
                    } else {
                        let err_msg = format!("`{}` -- expected `{message_type_rule}`", py_rule);
                        // ================= (LOG) =================
                        error!("{}", err_msg);
                        // =========================================
                        Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
                    }
                })
                .collect::<PyResult<Vec<_>>>()?)
        } else {
            let err_msg = format!(
                "`{}` -- expected `List` --> List [{message_type_rule}, {message_type_rule}, {message_type_rule}]",
                nested_rules
            );
            // ================= (LOG) =================
            error!("{}", err_msg);
            // =========================================
            Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
        }
    }
}
