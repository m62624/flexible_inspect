use super::*;
use std::hash::Hash;
/// Перечисление для хранения типа регулярного выражения
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RGX {
    Default,
    Fancy,
}
