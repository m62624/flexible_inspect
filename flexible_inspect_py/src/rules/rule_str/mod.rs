mod modifiers;
use super::*;

#[pyclass(name = "Rule")]
#[derive(Clone, Default, PartialEq)]
pub struct PyRule(Option<Rule>);

#[pymethods]
impl PyRule {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequeriment) -> Self {
        PyRule(Some(Rule::new(pattern, requirement.into())))
    }
}

impl TryFrom<&mut PyRule> for PyRule {
    type Error = PyErr;

    fn try_from(value: &mut PyRule) -> Result<Self, Self::Error> {
        let value = std::mem::take(value);
        if value.0.is_some() {
            Ok(value)
        } else {
            Err(PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
        }
    }
}

impl TryFrom<PyRule> for Rule {
    type Error = PyErr;

    fn try_from(value: PyRule) -> Result<Self, Self::Error> {
        value
            .0
            .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
    }
}
