use super::*;
use std::hash::{Hash, Hasher};
/// Перечисления для определения требований к строке
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub enum MatchRequirement {
    MustBeFound,
    MustNotBefound,
}

impl Hash for MatchRequirement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
