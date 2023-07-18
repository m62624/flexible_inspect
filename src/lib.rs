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
    // env_logger вызывается лишь раз
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
        py_module.add_class::<rule::MatchRequirement>()?;
        // py_module.add_class::<template_validator::TemplateValidator>()?;
        PyModule::from_code(_py, &base_error::export_base_error(), "", MODULE_NAME)?;
        Ok(())
    }
}
