use super::*;

#[pyclass(name = "RuleBytes")]
pub struct PyRuleBytes(RuleBytes);

#[pymethods]
impl PyRuleBytes {
    #[new]
    fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(RuleBytes::new(pattern, requirement.to_rust()))
    }
}
