use pyo3::prelude::*;
use std::collections::HashMap;
mod check_convert;
mod init;

#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    #[pyo3(get, set)]
    py_classes: HashMap<usize, PyObject>,
}

/// Перечисления для определения требований к строке
#[pyclass]
#[derive(Debug, Clone)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}

/// Структура для хранения вложенных строк
#[pyclass]
#[derive(Debug, Clone)]
pub struct Rule {
    inner: String,
    requirements: MatchRequirement,
    rules_for_the_rule: HashMap<String, Rule>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ContainerTree {
    selected_rules: regex::RegexSet,
}
#[pymethods]
impl Rule {
    /// Добавление дочерней строки
    pub fn extend(&mut self, key: String, child: Rule) {
        self.rules_for_the_rule.insert(key, child);
    }
}
