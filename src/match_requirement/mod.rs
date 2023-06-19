use super::*;
mod impl_hash;
/// Перечисления для определения требований к строке
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}
