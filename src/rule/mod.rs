use super::*;
mod getters;
/// Импорт модуля с имплементацией `__eq__`
mod impl_eq;
/// Импорт модуля с имплементацией `__hash__`
mod impl_hash;
pub mod regex_types;
use match_requirement::MatchRequirement;
use pyo3::{exceptions, types};

/// Структура для хранения вложенных строк
/// Ставим всё в Option, чтобы можно было использовать `take`.
/// `take` - забирает значение из переменной, а вместо него ставит `None`
/// тем самым мы сможем каждый раз перемещать значение из переменной в переменную без копирования

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Rule {
    #[pyo3(get)]
    /// Строка является Regex выражением
    inner: Option<(String, regex_types::RGX)>,
    #[pyo3(get)]
    /// Какое требование при нахождении совпадений
    requirement: Option<MatchRequirement>,
    #[pyo3(get)]
    /// Вложенные правила, которые будут проверяться, если данное правило сработало
    rules_for_the_rule: Option<Vec<Rule>>,
    // regex_set: Option<regex::RegexSet>,
}

#[pymethods]
impl Rule {
    #[new]
    /// Создание корня дерева
    pub fn new(inner: String, requirements: MatchRequirement) -> PyResult<Self> {
        Ok(Rule {
            inner: if check_convert::check::is_default_regex_fisrt_step(&inner) {
                Some((inner, regex_types::RGX::Default))
            } else if check_convert::check::is_fancy_regex_second_step(&inner) {
                Some((inner, regex_types::RGX::Fancy))
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                    "Expected `Regex` or `FancyRegex`, got `{}`",
                    inner
                )));
            },
            requirement: Some(requirements),
            rules_for_the_rule: None,
            // regex_set: None,
        })
    }
    /// Добавление дочерней строки
    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            // Итерируемся по списку для получения всех дочерних правил
            list.iter().map(|packed_rule| {
                if let Ok(rule) = packed_rule.extract::<Rule>() {
                    // Добавляем в вектор дочерних правил
                    if let Some(rules) = &mut self.rules_for_the_rule {
                        rules.push(rule);
                        Ok(())
                    } else {
                        self.rules_for_the_rule = Some(vec![rule]);
                        Ok(())
                    }
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",
                        packed_rule.get_type().name().unwrap(),
                        self.inner.as_ref().unwrap().0
                    )));
                }
            }).collect::<PyResult<Vec<_>>>()?;
            // self.regex_set = Self::get_regex_set(&self.rules_for_the_rule);
            // Возвращаем саму структуру
            return Ok(std::mem::take(self));
        }
        Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "`{}` -- Expected `List` --> List[Rule, Rule, Rule]",
            nested_rules.as_ref(py).get_type().name().unwrap()
        )))
    }

    /// Показать дерево конкретного правила
    pub fn show_tree(&self) {
        println!("{:#?}", self);
    }
}

impl Rule {
    pub fn make_regex_set(subrules: &Option<Vec<Rule>>) -> Option<regex::RegexSet> {
        if let Some(rules) = subrules {
            return Some(
                regex::RegexSet::new(
                    rules
                        .iter()
                        .filter_map(|rule| {
                            if let Some((inner, regex_type)) = &rule.inner {
                                if let regex_types::RGX::Default = regex_type {
                                    return Some(inner.as_str());
                                }
                                return None;
                            }
                            return None;
                        })
                        .collect::<Vec<&str>>(),
                )
                .unwrap(),
            );
        }
        None
    }
}
