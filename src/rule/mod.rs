use pyo3::{exceptions, types};
mod getters;
use super::*;

#[pyclass]
#[derive(Debug, Clone, Default)]
/// --> ExceptionContainer
pub struct Rule {
    str_raw: Option<RegexRaw>,
    requirement: Option<MatchRequirement>,
    subrules: Option<Vec<Rule>>,
}
#[derive(Debug, Clone)]
/// --> Rule
pub enum RegexRaw {
    DefaultR(Box<str>),
    FancyR(Box<str>),
}
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
/// --> Rule
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}
#[pymethods]
impl Rule {
    #[new]
    pub fn new(pattern: String, requirements: MatchRequirement) -> PyResult<Self> {
        Ok(Self {
            str_raw: Some(Self::check_regex(pattern)?),
            requirement: Some(requirements),
            subrules: None,
        })
    }
    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            // Итерируемся по списку для получения всех дочерних правил
            list.iter().map(|packed_rule| {
                if let Ok(rule) = packed_rule.extract::<Rule>() {
                    // Добавляем в вектор дочерних правил
                    if let Some(rules) = &mut self.subrules {
                        rules.push(rule);
                        Ok(())
                    } else {
                        self.subrules = Some(vec![rule]);
                        Ok(())
                    }
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",
                        packed_rule.get_type().name().unwrap(),
                        self.str_raw.as_ref().unwrap().get_str()
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
}
impl Rule {
    pub fn check_regex(pattern: String) -> PyResult<RegexRaw> {
        if regex::Regex::new(&pattern).is_ok() {
            return Ok(RegexRaw::DefaultR(pattern.into_boxed_str()));
        } else if fancy_regex::Regex::new(&pattern).is_ok() {
            return Ok(RegexRaw::FancyR(pattern.into_boxed_str()));
        }
        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "Expected `Regex` or `FancyRegex`, got `{}`",
            pattern
        )));
    }
}
