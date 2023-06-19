use pyo3::prelude::*;
use std::collections::HashMap;
mod check_convert;
mod custom_error;
mod init;
mod match_requirement;
mod rule;
mod template_validator;
// имя модуля для `Python`
pub const MODULE_NAME: &str = "pystval";
// имя атрибута, где хранится само сообщение и **extra переменные из класса Python
pub const MESSAGE_WITH_EXTRA_FROM_CLASS_PY: &str = "message";
// имя атрибута, где хранится регулярные выражения из класса Python
pub const RULES_FROM_CLASS_PY: &str = "rules";
// имя атрибута, где хранится **extra переменные для заполнения из класса Python
pub const EXTRA_FROM_CLASS_PY: &str = "extra";
// имя класса ошибки (базовый шаблон) для `Python`
pub const BASE_ERROR: &str = "PystvalError";

// Импортируем всё необходимое в `Python`
#[cfg(not(tarpaulin_include))]
mod export {
    use super::*;
    #[pymodule]
    fn pystval(_py: Python<'_>, py_module: &PyModule) -> PyResult<()> {
        PyModule::from_code(
            _py,
            &custom_error::py_code_base_exception(),
            "",
            MODULE_NAME,
        )?;
        py_module.add_class::<match_requirement::MatchRequirement>()?;
        py_module.add_class::<rule::Rule>()?;
        py_module.add_class::<template_validator::TemplateValidator>()?;
        Ok(())
    }
}
