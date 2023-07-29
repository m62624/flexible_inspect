use super::*;

#[pyclass(name = "Rule")]
pub struct PyRule(Rule);

#[pymethods]
impl PyRule {
    #[new]
    fn new(pattern: String, requirement: PyMatchRequirement) -> Self {
        Self(Rule::new(pattern, requirement.to_rust()))
    }
}
