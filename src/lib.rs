//! `Pystval` - `Rust` библиотека для `Python`. Выполняет валидацию данных (строки) с помощью регулярных выражений

//===============================================================================
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
//===============================================================================

// Проверка и конвертация данных из Python в Rust и обратно
mod check_convert;
// Компоненты необходимые для конструктора `TemplateValidator`
mod init;
// Базовый класс для кастомных ошибок
mod base_error;
// Создание и получение данных ошибок
mod make_errors;
// Компоненты для метода, где проходит сама валидация
mod validate;
// Все юнит тесты выненсены в отдельный модуль
mod unit_tests;

// Отвечает за взаимодействие сборщика мусора СPython с Rust
use pyo3::gc::{PyTraverseError, PyVisit};
// Используем для получения данных из Python
use pyo3::{prelude::*, types};
use std::{collections::HashMap, str};
/// Перечечисление, где даны варианты действия при положительном результате регулярных выражений
#[pyclass]
#[derive(Debug, Clone)]
pub enum It {
    /// * `MustBeFoundHere` - Должно быть найдено, иначе будет вызвано исключение
    MustBeFoundHere,
    /// * `NotToBeFoundHere` - Не должно быть найдено, иначе будет вызвано исключение
    NotToBeFoundHere,
}

/// Структура для хранения ошибок и статуса
#[derive(Debug, Clone)]
pub struct RuleStatus {
    id: usize,
    status: It,
}

impl RuleStatus {
    pub fn new(id: usize, status: It) -> Self {
        Self { id, status }
    }
}

/// Структруа для хранения ошибок и регулярных выражений\
/// Является шаблоном для создание *уникальных* валидаторов
#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    // хранит все ошибки ( KEY: `ID` и VALUE: `PyError` )
    python_classes: HashMap<usize, PyObject>,
    // хранит default regex (KEY `Regex` и VALUE `ID CLASS & STATUS`)
    all_simple_rules: HashMap<String, RuleStatus>,
    // хранит fancy regex (KEY `Regex` и VALUE `ID CLASS & STATUS`)
    all_hard_rules: HashMap<String, RuleStatus>,
    // Собираем все default regex и *одним проходом* проверяем всё
    selected_simple_rules: regex::RegexSet,
}

// Реализация методов для TemplateValidator которые будут доступны в `Python`
#[pymethods]
impl TemplateValidator {
    /// Создаем экземлпяр с заданными параметрами проверки\
    /// Принимает `PyDict {class,status_throw}`\
    /// **Может принимать сразу `class` без экземпляра**
    #[new]
    pub fn __new__(flags: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| -> PyResult<Self> {
            let mut python_classes: HashMap<usize, PyObject> = HashMap::new();
            let mut all_simple_rules: HashMap<String, RuleStatus> = HashMap::new();
            let mut all_hard_rules: HashMap<String, RuleStatus> = HashMap::new();
            let mut selected_simple_rules: Vec<String> = Vec::new();
            init::data_unpackaging(
                py,
                flags,
                &mut python_classes,
                &mut all_simple_rules,
                &mut all_hard_rules,
                &mut selected_simple_rules,
            )?;
            let rsl_info = Self {
                all_hard_rules,
                all_simple_rules,
                python_classes,
                selected_simple_rules: regex::RegexSet::new(selected_simple_rules).unwrap(),
            };
            // dbg!(&rsl_info);
            Ok(rsl_info)
        })
    }

    /// Служит для запуска async метода
    #[cfg(not(tarpaulin_include))]
    #[pyo3(name = "validate")]
    fn validate<'py>(&self, py: Python<'py>, text_bytes: &types::PyBytes) -> PyResult<&'py PyAny> {
        let unsafe_self = unsafe { &*(self as *const Self) };
        let text = check_convert::convert::bytes_to_string_utf8(text_bytes.as_bytes())?;
        pyo3_asyncio::async_std::future_into_py(py, async move {
            unsafe_self.core_validate(text)?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }
    //================== (РАБОТАЕТ ТОЛЬКО С `C API` (CPYTHON))==================
    /*
    Метод `__traverse__` используется для рекурсивного обхода объекта и уведомления `Python` о всех
    вложенных объектах, которые должны быть добавлены в механизм управления памятью `Python`. В этой
    реализации метод `__traverse__` проходится по всем значениям в хэш-таблице `python_classes` и вызывает
    `visit.call()` для каждого объекта типа `PyObject` в свойстве `error` объекта типа `RuleStatus`.
    Это гарантирует, что объекты типа `PyObject` не будут освобождены Python'ом до тех пор,
    пока они не перестанут использоваться в `Rust`.
     */
    #[cfg(not(tarpaulin_include))]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for class_py in self.python_classes.values() {
            visit.call(class_py)?;
        }
        Ok(())
    }

    /*
    Метод `__clear__` используется для освобождения памяти, занятой объектом. В этой реализации метод
    `__clear__` очищает хэш-таблицу `python_classes`, что приводит к уменьшению счетчика ссылок на каждый
    объект типа PyObject в свойстве error объекта типа `RuleStatus`. Если количество ссылок на объект
    достигнет нуля, он будет автоматически удален Python'ом.
     */
    #[cfg(not(tarpaulin_include))]
    fn __clear__(&mut self) {
        // Удаляем все значения из HashMap, тем самым снижая счетчик ссылок на PyObject
        self.python_classes.clear();
    }
    //================== (РАБОТАЕТ ТОЛЬКО С `C API` (CPYTHON))==================
}

// Импортируем всё необходимое в `Python`
#[cfg(not(tarpaulin_include))]
mod export {

    use super::*;
    #[pymodule]
    fn pystval(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_class::<TemplateValidator>()?;
        m.add_class::<It>()?;
        Ok(())
    }
}
