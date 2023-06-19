use std::f32::consts::E;

use super::*;
/// Импорт модуля с имплементацией `__eq__`
mod impl_eq;
/// Импорт модуля с имплементацией `__hash__`
mod impl_hash;
use match_requirement::MatchRequirement;
use pyo3::{exceptions, types};
/// Структура для хранения вложенных строк
/// Ставим всё в Option, чтобы можно было использовать `take`
/// `take` - забирает значение из переменной, а вместо него ставит `None`
/// тем самым мы сможем каждый раз перемещать значение из переменной в переменную без копирования
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Rule {
    /// Строка является Regex выражением
    inner: Option<String>,
    /// Какое требование при нахождении совпадений
    requirements: Option<MatchRequirement>,
    /// Вложенные правила, которые будут проверяться, если данное правило сработало
    rules_for_the_rule: Option<Vec<Rule>>,
}

#[pymethods]
impl Rule {
    #[new]
    /// Создание корня дерева
    pub fn new(inner: String, requirements: MatchRequirement) -> Self {
        Rule {
            inner: Some(inner),
            requirements: Some(requirements),
            rules_for_the_rule: Some(Vec::new()),
        }
    }
    /// Добавление дочерней строки
    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            // Итерируемся по списку для получения всех дочерних правил
            for list in list.iter() {
                // Проверяем, что это правило
                if let Ok(rule) = list.extract::<Rule>() {
                    // Добавляем в вектор дочерних правил
                    self.rules_for_the_rule.as_mut().unwrap().push(rule);
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>("Expected Rule"));
                }
            }
            // Возвращаем саму структуру
            return Ok(std::mem::take(self));
        }
        return Err(PyErr::new::<exceptions::PyTypeError, _>("Expected List"));
    }
    /// Показать дерево конкретного правила
    pub fn show_tree(&self) {
        println!("{:#?}", self);
    }
}
