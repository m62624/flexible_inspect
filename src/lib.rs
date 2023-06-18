use pyo3::prelude::*;
use std::collections::HashMap;
mod init;
/// Перечисления для определения требований к строке
#[pyclass]
#[derive(Debug, Clone)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    // хранит все ошибки ( KEY: `ID` и VALUE: `PyError` )
    py_error_classes: HashMap<usize, PyObject>,
    rules_from_py_error_classes: HashMap<usize, Rule>,
}

/// Структура для хранения вложенных строк
#[pyclass]
#[derive(Debug, Clone)]
pub struct Rule {
    pub inner: String,
    pub requirements: MatchRequirement,
    pub rule_for_the_rule: HashMap<String, Rule>,
}

/// Итератор для обхода дерева в глубину
pub struct RuleIterator<'a> {
    stack: Vec<&'a Rule>,
}
