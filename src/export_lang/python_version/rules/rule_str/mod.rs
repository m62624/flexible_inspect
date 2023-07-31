mod modifiers;

use super::*;

#[pyclass(name = "Rule")]
#[derive(Default, Clone, Debug)]
pub struct PyRule(Rule);

#[pymethods]
impl PyRule {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(Rule::new(pattern, requirement.into()))
    }
}

impl From<PyRule> for Rule {
    fn from(value: PyRule) -> Self {
        value.0
    }
}
