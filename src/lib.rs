/// Модуль для юнит тестов
#[cfg(test)]
mod unit_tests;
//=====================================================================
use pyo3::exceptions::{self, PyException};
use pyo3::prelude::*;
use pyo3::types;
//=====================================================================
use log::{debug, error};
use std::sync::Once;
//=====================================================================
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
//=====================================================================
/// Модуль для создания ошибок
mod base_error;
/// Модуль для получения совпадений
mod captures;
/// Модуль для хранения классов и их правил
mod cartridge;
/// Модуль для создания ошибок
mod custom_error;
/// Модуль для создания правил
mod rule;
/// Модуль для создания валидатора
mod template_validator;
// ============================= CONST ================================
// имя модуля для `Python`
pub const MODULE_NAME: &str = "pystval";
// имя атрибута, где хранится само сообщение и **extra переменные из класса `Python`
pub const MESSAGE_WITH_EXTRA_FROM_CLASS_PY: &str = "message";
// имя атрибута, где хранится регулярные выражения из класса `Python`
pub const RULES_FROM_CLASS_PY: &str = "rules";
// имя атрибута, где хранится **extra переменные для заполнения из класса `Python`
pub const EXTRA_FROM_CLASS_PY: &str = "extra";
// имя класса ошибки (базовый шаблон) для `Python`
pub const BASE_ERROR: &str = "PystvalException";
/// Для однократной инициализации в FFI
static INIT: Once = Once::new();
// =====================================================================

#[cfg(not(tarpaulin_include))]
/// Инициализация логгера
pub fn init_logger() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

// =====================================================================

// Импортируем всё необходимое в `Python`
#[cfg(not(tarpaulin_include))]
mod export {
    use super::*;

    #[pymodule]
    pub fn pystval(_py: Python<'_>, py_module: &PyModule) -> PyResult<()> {
        py_module.add_class::<rule::Rule>()?;

        // ================= (LOG) =================
        debug!("successfully importing `Rule` into python");
        // =========================================

        py_module.add_class::<rule::MatchRequirement>()?;

        // ================= (LOG) =================
        debug!("successfully importing `MatchRequirement` into python");
        // =========================================

        py_module.add_class::<template_validator::TemplateValidator>()?;

        // ================= (LOG) =================
        debug!("successfully importing `TemplateValidator` into python");
        // =========================================

        PyModule::from_code(_py, &base_error::export_base_error(), "", MODULE_NAME)?;

        // ================= (LOG) =================
        debug!("successfully importing `PystvalException` into python");
        // =========================================

        Ok(())
    }
}
