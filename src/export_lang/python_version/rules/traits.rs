use crate::core::rules::traits::{RuleBase, RuleModifiers};

use super::*;
use log::error;
use pyo3::{exceptions, FromPyObject, Python};
pub trait PyRuleBase {
    type RulTypeRust: RuleBase + RuleModifiers;
    fn to_rust(&mut self) -> Self::RulTypeRust;
}

pub trait PyRuleModifiers: PyRuleBase {
    type RuleTypePy;
    fn to_rust_for_extend<ConvertToRust>(
        &mut self,
        py: Python,
        nested_rules: PyObject,
        message_type_rule: &str,
    ) -> PyResult<Vec<Self::RulTypeRust>>
    where
        ConvertToRust: for<'a> FromPyObject<'a> + PyRuleBase<RulTypeRust = Self::RulTypeRust>,
    {
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            Ok(list
                .iter()
                .map(|py_rule| {
                    if let Ok(mut rule) = py_rule.extract::<ConvertToRust>() {
                        Ok(rule.to_rust())
                    } else {
                        let err_msg = format!("`{}` -- expected `{message_type_rule}`", py_rule);
                        // ================= (LOG) =================
                        error!("{}", err_msg);
                        // =========================================
                        return Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg));
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
            return Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg));
        }
    }
}
