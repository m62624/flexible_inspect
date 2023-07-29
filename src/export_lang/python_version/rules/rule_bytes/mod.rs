mod modifiers;

use super::*;

#[pyclass(name = "RuleBytes")]
#[derive(Default, Clone)]
pub struct PyRuleBytes(RuleBytes);

#[pymethods]
impl PyRuleBytes {
    #[new]
    fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(RuleBytes::new(pattern, requirement.to_rust()))
    }
}

impl PyRuleBytes {
    fn to_rust(&mut self) -> RuleBytes {
        std::mem::take(&mut self.0)
    }
}
