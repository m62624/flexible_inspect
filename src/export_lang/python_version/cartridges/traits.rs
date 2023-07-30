use crate::export_lang::python_version::rules::traits::PyRuleBase;
use log::error;
use pyo3::{exceptions, prelude::*, types};

pub trait PyCartridgeBase {
    type CartridgeType;
    type RuleType: PyRuleBase;
    fn to_rust(&mut self) -> Self::CartridgeType;
    fn new(
        &mut self,
        py: Python,
        root_rules: PyObject,
        message_type_rule: &str,
    ) -> PyResult<Self::CartridgeType> {
        if let Ok(list) = root_rules.downcast::<types::PyList>(py) {
            Ok(())
        } else {
            let err_msg = format!(
                "`{}` -- expected `List` --> List [{message_type_rule}, {message_type_rule}, {message_type_rule}]",
                root_rules
            );
            // ================= (LOG) =================
            error!("{}", err_msg);
            // =========================================
            return Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg));
        }
    }
}
