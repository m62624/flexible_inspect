use super::*;
mod impl_eq;
mod impl_hash;
#[pyclass]
#[derive(Debug, Clone)]
pub struct ContainerTree {
    rules: rule::Rule,
    selected_rules: regex::RegexSet,
}
