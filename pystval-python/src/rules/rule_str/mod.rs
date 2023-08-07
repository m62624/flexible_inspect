mod modifiers;
use super::*;

#[pyclass(name="Rule")]
#[derive(Clone, Default)]
pub struct PyRule(Rule);

#[pymethods]
impl PyRule {
    #[new]
    pub fn new(pattern: String, match_requirement: PyMatchRequeriment) -> Self {
        PyRule(Rule::new(pattern, match_requirement.into()))
    }
}

impl From<PyRule> for Rule {
    fn from(value: PyRule) -> Self {
        value.0
    }
}
