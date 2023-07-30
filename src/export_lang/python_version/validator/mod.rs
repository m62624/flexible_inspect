mod validator_bytes;
mod validator_str;

use super::cartridges::traits::PyCartridgeBase;
use super::rules::traits::PyRuleBase;
use crate::core::cartridges::CartridgeBase;
use crate::core::rules::rule_bytes::RuleBytes;
use crate::core::rules::traits::RuleBase;
use crate::core::validator::TemplateValidator;
use crate::core::{cartridges::Cartridge, rules::rule_str::Rule};
use async_trait::async_trait;
use log::error;
use pyo3::{exceptions, pyclass};
use pyo3::{prelude::*, types};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

#[async_trait]
pub trait PyTemplateValidatorBase: PyCartridgeBase {
    type CartridgeTypeRust: PyCartridgeBase;

    
    fn _to_rust_for_extend<ConvertToRust>(
        py: Python,
        nested_rules: PyObject,
        message_type_rule: &str,
    ) -> PyResult<Vec<Self::CartridgeTypeRust>>
    where
        ConvertToRust:
            for<'a> FromPyObject<'a> + PyCartridgeBase<CartridgeType = Self::CartridgeTypeRust>,
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
