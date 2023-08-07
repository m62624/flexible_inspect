mod modifiers;
use super::*;

#[pyclass(name = "RuleBytes")]
#[derive(Clone, Default)]
pub struct PyRuleBytes(RuleBytes);

#[pymethods]
impl PyRuleBytes {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequeriment) -> Self {
        PyRuleBytes(RuleBytes::new(pattern, requirement.into()))
    }
}

impl From<PyRuleBytes> for RuleBytes {
    fn from(value: PyRuleBytes) -> Self {
        value.0
    }
}
