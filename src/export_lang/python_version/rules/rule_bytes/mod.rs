mod modifiers;

use super::{traits::PyRuleBase, *};

#[pyclass(name = "RuleBytes")]
#[derive(Default, Clone, Debug)]
pub struct PyRuleBytes(RuleBytes);

#[pymethods]
impl PyRuleBytes {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(RuleBytes::new(pattern, requirement.to_rust()))
    }
}

impl PyRuleBase for PyRuleBytes {
    type RulTypeRust = RuleBytes;

    fn to_rust(&mut self) -> RuleBytes {
        std::mem::take(&mut self.0)
    }
}
