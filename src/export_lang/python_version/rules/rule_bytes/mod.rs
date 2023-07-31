mod modifiers;

use super::*;

#[pyclass(name = "RuleBytes")]
#[derive(Default, Clone, Debug)]
pub struct PyRuleBytes(RuleBytes);

#[pymethods]
impl PyRuleBytes {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(RuleBytes::new(pattern, requirement.into()))
    }
}

impl From<PyRuleBytes> for RuleBytes {
    fn from(value: PyRuleBytes) -> Self {
        value.0
    }
}
