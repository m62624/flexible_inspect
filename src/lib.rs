use pyo3::prelude::*;
use std::collections::HashMap;
mod check_convert;
mod init;
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
    rule_for_the_rule: HashMap<String, Rule>,
}

#[pymethods]
impl Rule {
    /// Добавление дочерней вложенной строки
    pub fn extend(&mut self, key: String, child: Rule) {
        self.rule_for_the_rule.insert(key, child);
    }
}
