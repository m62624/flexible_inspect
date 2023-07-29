use super::*;
use crate::core::rules::traits::RuleModifiers;
use log::error;
use pyo3::{exceptions, PyResult};

#[pymethods]
impl PyRuleBytes {
    pub fn extend(&mut self, py: Python, nested_rules: PyObject) -> PyResult<Self> {
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            self.0 = self.0.extend(
                list.iter()
                    .map(|rule| {
                        if let Ok(mut py_rule) = rule.extract::<PyRuleBytes>() {
                            Ok(py_rule.to_rust())
                        } else {
                            let err_msg = format!("`{}` -- expected `RuleBytes`", rule);
                            // ================= (LOG) =================
                            error!("{}", err_msg);
                            // =========================================
                            return Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg));
                        }
                    })
                    .collect::<PyResult<Vec<_>>>()?,
            );
        } else {
            let err_msg = format!(
                "`{}` -- expected `List` --> List [RuleBytes, RuleBytes, RuleBytes]",
                nested_rules
            );
            // ================= (LOG) =================
            error!("{}", err_msg);
            // =========================================
            return Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg));
        }

        Ok(std::mem::take(self))
    }
}
