mod modifiers;

use super::{traits::PyRuleBase, *};

#[pyclass(name = "RuleBytes")]
#[derive(Default, Clone)]
pub struct PyRuleBytes(RuleBytes);

impl PyRuleBase for PyRuleBytes {
    type RulTypeRust = RuleBytes;

    fn to_rust(&mut self) -> RuleBytes {
        std::mem::take(&mut self.0)
    }
}

#[pymethods]
impl PyRuleBytes {
    #[new]
    fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(RuleBytes::new(pattern, requirement.to_rust()))
    }
}
