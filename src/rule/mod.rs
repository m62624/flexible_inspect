use super::*;
use std::collections::hash_map::RandomState;
mod impl_eq;
mod impl_hash;
/// Структура для хранения вложенных строк
#[pyclass]
#[derive(Debug, Clone)]
pub struct Rule {
    inner: String,
    requirements: match_requirement::MatchRequirement,
    rules_for_the_rule: HashMap<String, Rule>,
}
#[pymethods]
impl Rule {
    #[new]
    /// Создание корня дерева
    pub fn new(inner: String, requirements: match_requirement::MatchRequirement) -> Self {
        Rule {
            inner,
            requirements,
            rules_for_the_rule: HashMap::new(),
        }
    }
    /// Добавление дочерней строки
    pub fn extend(&mut self, key: String, child: Rule) {
        self.rules_for_the_rule.insert(key, child);
    }
}
