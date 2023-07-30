mod modifiers;

use super::{traits::PyRuleBase, *};

#[pyclass(name = "Rule")]
#[derive(Default, Clone, Debug)]
pub struct PyRule(Rule);

#[pymethods]
impl PyRule {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(Rule::new(pattern, requirement.to_rust()))
    }
}

impl PyRuleBase for PyRule {
    type RulTypeRust = Rule;

    fn to_rust(&mut self) -> Rule {
        std::mem::take(&mut self.0)
    }
}
