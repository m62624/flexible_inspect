pub mod validator_bytes;
pub mod validator_str;

use super::cartridges::traits::PyCartridgeBase;
use crate::core::rules::rule_bytes::RuleBytes;
use crate::core::validator::TemplateValidator;
use crate::core::validator::ValidatorBase;
use crate::core::{cartridges::Cartridge, rules::rule_str::Rule};
use crate::export_lang::python_version::cartridges::cartridge_bytes::PyCartridgeBytes;
use crate::export_lang::python_version::cartridges::cartridge_str::PyCartridge;
use log::error;
use pyo3::{exceptions, pyclass};
use pyo3::{prelude::*, types};
use std::sync::Arc;

pub trait PyTemplateValidatorBase {
    type CartridgeTypeRust;
    fn _to_rust_for_new<ConvertToRust>(
        py: Python,
        nested_rules: PyObject,
        message_type_cartridge: &str,
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
                        let err_msg = format!("`{}` -- expected `{message_type_cartridge}`", py_rule);
                        // ================= (LOG) =================
                        error!("{}", err_msg);
                        // =========================================
                        return Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg));
                    }
                })
                .collect::<PyResult<Vec<_>>>()?)
        } else {
            let err_msg = format!(
                "`{}` -- expected `List` --> List [{message_type_cartridge}, {message_type_cartridge}, {message_type_cartridge}]",
                nested_rules
            );
            // ================= (LOG) =================
            error!("{}", err_msg);
            // =========================================
            return Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg));
        }
    }
}
