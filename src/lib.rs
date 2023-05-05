mod export_py;
mod global_const;
mod init;
mod make_py_error;
use global_const::*;
use pyo3::prelude::*;
use pyo3::types;
use pyo3::ToPyObject;
use regex::Regex;
use regex::RegexSet;
use std::collections::HashMap;
use std::str;
use uuid::Uuid;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Validator {
    // Храним KEY: `regex` и VALUE: `uuid`
    selection_rules: HashMap<String, String>,
    // Храним KEY: `uuid` и VALUE: `ClassError`
    python_classes: HashMap<String, PyObject>,
    // Собираем все regex и *одним проходом* проверяем все регулярки
    all_rules: RegexSet,
}
// Методы которые будут использоваться в `Python`
#[pymethods]
impl Validator {
    #[new]
    fn py_constructor(flags_errors: PyObject) -> PyResult<Self> {
        // Храним все регулярные выражения вместе с классом ошибки
        let mut python_classes = HashMap::new();
        // Храним все регулярные выражения, но будем отбирать и создавать их, только,
        // если по ним было найденно совпадение
        let mut selection_rules = HashMap::new();
        // Храним все правила, чтобы разом провести
        // поиск по всем регулярным выражениям за *одни раз*
        let mut all_rules = vec![];
        Python::with_gil(|py| {
            // Получаем каждый класс с коллекций
            for py_class in flags_errors.downcast::<types::PyList>(py)? {
                // Класс ошибки из`Python`
                let py_class = py_class.downcast::<types::PyType>()?;
                // Уникальный UUID, для каждого класса
                let id = Uuid::new_v4().to_string();
                init::regex::collection_regex(
                    &mut selection_rules,
                    &mut all_rules,
                    &py_class,
                    id.to_owned(),
                )?;
                python_classes.insert(id, py_class.to_object(py));
            }
            Ok::<(), PyErr>(())
        })?;
        let x = Self {
            selection_rules,
            python_classes,
            all_rules: RegexSet::new(&all_rules).unwrap(),
        };
        dbg!(&x);
        Ok(x)
    }
}
// методы которые являются только `Rust` методами или вспомагательными
// для `Python` методов
impl Validator {}
