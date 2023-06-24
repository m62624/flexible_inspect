use super::*;
mod getters;
/// Импорт модуля с имплементацией `__hash__`
mod impl_hash;
mod match_requirement;
mod regex_set;
pub mod regex_types;
pub use match_requirement::MatchRequirement;
use pyo3::{exceptions, types};

/// Структура для хранения вложенных строк
/// Ставим всё в Option, чтобы можно было использовать `take`.
/// Функция `take` - забирает значение из переменной, а вместо него ставит `None`
/// тем самым мы можем каждый раз перемещать значение из переменной в переменную без копирования
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Rule {
    /// Строка является Regex выражением
    rule_raw: Option<(Box<str>, regex_types::RGX, MatchRequirement)>,
    #[pyo3(get)]
    /// Вложенные правила, которые будут проверяться, если данное правило сработало
    rules_for_the_rule: Option<Vec<Rule>>,
}

#[pymethods]
impl Rule {
    #[new]
    /// Создание корня дерева
    pub fn new(rule_raw: String, requirements: MatchRequirement) -> PyResult<Self> {
        Ok(Rule {
            rule_raw: if check_convert::check::is_default_regex_fisrt_step(&rule_raw) {
                Some((
                    rule_raw.into_boxed_str(),
                    regex_types::RGX::Default,
                    requirements,
                ))
            } else if check_convert::check::is_fancy_regex_second_step(&rule_raw) {
                Some((
                    rule_raw.into_boxed_str(),
                    regex_types::RGX::Fancy,
                    requirements,
                ))
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                    "Expected `Regex` or `FancyRegex`, got `{}`",
                    rule_raw
                )));
            },
            rules_for_the_rule: None,
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
                        self.rule_raw.as_ref().unwrap().0
                    )));
                }
            }).collect::<PyResult<Vec<_>>>()?;
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
