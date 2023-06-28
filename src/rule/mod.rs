mod getters;
use pyo3::types::PyList;
use pyo3::{exceptions, prelude::*};
#[cfg(test)]
mod unit_tests;

#[pyclass]
#[derive(Debug, Clone, Default)]
/// --> ExceptionContainer
pub struct Rule {
    str_raw: Option<RegexRaw>,
    requirement: Option<MatchRequirement>,
    subrules: Option<Subrules>,
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

#[derive(Debug, Clone)]
pub struct Subrules {
    default_r_vec: Vec<Rule>,
    fancy_r_vec: Vec<Rule>,
}

#[pymethods]
impl Rule {
    #[new]
    pub fn new(pattern: String, requirements: MatchRequirement) -> PyResult<Self> {
        Ok(Self {
            str_raw: Some(RegexRaw::new(pattern)?),
            requirement: Some(requirements),
            subrules: None,
        })
    }

    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        let (mut default_r_vec, mut fancy_r_vec) = (Vec::new(), Vec::new());
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<PyList>(py) {
            // Итерируемся по списку для получения всех дочерних правил
            list.iter().map(|packed_rule| {
                if let Ok(rule) = packed_rule.extract::<Rule>() {
                    match rule.get_str_raw()? {
                        RegexRaw::DefaultR(_) => default_r_vec.push(rule),
                        RegexRaw::FancyR(_) => fancy_r_vec.push(rule),
                    }
                    Ok(())
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",
                        packed_rule.get_type().name().unwrap(),
                        self.str_raw.as_ref().unwrap().get_str()
                    )));
                }
            }).collect::<PyResult<Vec<_>>>()?;
            if !default_r_vec.is_empty() || !fancy_r_vec.is_empty() {
                self.subrules = Some(Subrules::new(default_r_vec, fancy_r_vec));
            }
            return Ok(std::mem::take(self));
        }
        Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "`{}` -- Expected `List` --> List[Rule, Rule, Rule]",
            nested_rules.as_ref(py).get_type().name().unwrap()
        )))
    }
}
impl Rule {
    fn absence_error() -> PyErr {
        PyErr::new::<exceptions::PyValueError, _>(format!(
            "* If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:
    
           x = Rule(\"X\")
           x.extend(Rule(\"Y\"))
           
           * Please use this syntax:
           
           x = Rule(\"X\").extend(Rule(\"Y\"))
           * or 
           x = Rule(\"X\")
           x = x.extend(Rule(\"Y\"))"
        ))
    }
}

impl RegexRaw {
    pub fn new(pattern: String) -> PyResult<RegexRaw> {
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

impl Subrules {
    pub fn new(default_r_vec: Vec<Rule>, fancy_r_vec: Vec<Rule>) -> Self {
        Self {
            default_r_vec,
            fancy_r_vec,
        }
    }
}
