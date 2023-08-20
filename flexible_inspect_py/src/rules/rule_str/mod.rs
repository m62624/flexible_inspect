mod modifiers;
use super::*;

#[pyclass(name = "Rule")]
#[derive(Clone, Default)]
pub struct PyRule(Rule);

#[pymethods]
impl PyRule {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequeriment) -> Self {
        PyRule(Rule::new(pattern, requirement.into()))
    }
}

impl From<PyRule> for Rule {
    fn from(value: PyRule) -> Self {
        value.0
    }
}
