use super::*;
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
/// --> Rule
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}
