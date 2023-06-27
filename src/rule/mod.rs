mod getters;
mod match_requirement;
mod regex_raw;
mod subrules;
mod take_self_error;
pub use match_requirement::MatchRequirement;
use pyo3::types::PyList;
use pyo3::{exceptions, prelude::*};
pub use regex_raw::RegexRaw;
pub use subrules::Subrules;
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
