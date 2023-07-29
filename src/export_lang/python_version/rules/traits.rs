use crate::core::rules::traits::{RuleBase, RuleModifiers};

use super::*;
use log::error;
use pyo3::{exceptions, FromPyObject, Python};
pub trait PyRuleBase {
    type RulTypeRust: RuleBase + RuleModifiers;
    fn to_rust(&mut self) -> Self::RulTypeRust;
}

pub trait PyRuleModifiers: PyRuleBase {
    type PyRuleType;

    /// modifier to set the match counter, condition counter == match
    fn _counter_is_equal(&mut self, count: usize) -> Self::PyRuleType;
    fn _counter_more_than(&mut self, count: usize) -> Self::PyRuleType;

    fn _counter_less_than(&mut self, count: usize) -> Self::PyRuleType;
    fn _mode_all_rules_for_at_least_one_match(&mut self) -> Self::PyRuleType;
    fn _mode_at_least_one_rule_for_all_matches(&mut self) -> Self::PyRuleType;
    fn _mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::PyRuleType;
    fn _to_rust_for_extend<ConvertToRust>(
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
