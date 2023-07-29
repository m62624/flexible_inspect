mod modifiers;

use super::*;

#[pyclass(name = "Rule")]
#[derive(Default, Clone)]
pub struct PyRule(Rule);

#[pymethods]
impl PyRule {
    #[new]
    fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(Rule::new(pattern, requirement.to_rust()))
    }
}

impl PyRule {
    fn to_rust(&mut self) -> Rule {
       std::mem::take(&mut self.0)
    }
}
