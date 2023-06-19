use super::*;
/// Импорт модуля с имплементацией `__eq__`
mod impl_eq;
/// Импорт модуля с имплементацией `__hash__`
mod impl_hash;
use match_requirement::MatchRequirement;
use pyo3::{exceptions, types};
/// Структура для хранения вложенных строк
/// Ставим всё в Option, чтобы можно было использовать `take`.
/// `take` - забирает значение из переменной, а вместо него ставит `None`
/// тем самым мы сможем каждый раз перемещать значение из переменной в переменную без копирования
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Rule {
    #[pyo3(get, set)]
    /// Строка является Regex выражением
    inner: Option<String>,
    #[pyo3(get, set)]
    /// Какое требование при нахождении совпадений
    requirement: Option<MatchRequirement>,
    #[pyo3(get, set)]
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
            requirement: Some(requirements),
            rules_for_the_rule: Some(Vec::new()),
        }
    }
    /// Добавление дочерней строки
    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            // Итерируемся по списку для получения всех дочерних правил
            for packed_rule in list.iter() {
                // Проверяем, что это правило
                if let Ok(rule) = packed_rule.extract::<Rule>() {
                    // Добавляем в вектор дочерних правил
                    self.rules_for_the_rule.as_mut().unwrap().push(rule);
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "`{}` -- Expected `Rule` (from parent `{}`) --> List[Rule,Rule,Rule]",
                        packed_rule.get_type().name().unwrap(),
                        self.inner.as_ref().unwrap()
                    )));
                }
            }
            // Возвращаем саму структуру
            return Ok(std::mem::take(self));
        }
        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "`{}` -- Expected `List` --> List[Rule,Rule,Rule]",
            nested_rules.as_ref(py).get_type().name().unwrap()
        )));
    }

    /// Показать дерево конкретного правила
    pub fn show_tree(&self) {
        println!("{:#?}", self);
    }
}

impl Rule {
    /// Получаем ближайшего родителя в дереве
    pub fn find_parent(&self, rule: &Rule) -> Option<Rule> {
        // Проверяем, является ли текущее правило родителем
        if let Some(rules) = &self.rules_for_the_rule {
            if rules.contains(rule) {
                return Some(self.clone());
            }
        }
        // Рекурсивно проверяем каждое дочернее правило
        if let Some(rules) = &self.rules_for_the_rule {
            for child_rule in rules {
                if let Some(parent) = child_rule.find_parent(rule) {
                    return Some(parent);
                }
            }
        }
        // Если не найден родитель, возвращаем None
        None
    }
}
