mod modifiers;
use super::*;

#[pyclass(name = "RuleBytes")]
#[derive(Clone, Default)]
pub struct PyRuleBytes(RuleBytes);

#[pymethods]
impl PyRuleBytes {
    #[new]
    pub fn new(pattern: String, match_requirement: PyMatchRequeriment) -> Self {
        PyRuleBytes(RuleBytes::new(pattern, match_requirement.into()))
    }
}

impl From<PyRuleBytes> for RuleBytes {
    fn from(value: PyRuleBytes) -> Self {
        value.0
    }
}
