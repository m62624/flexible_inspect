use super::rule::{SimpleRules, Subrules as RootRules};
use super::*;

#[derive(Debug)]

pub struct Cartridge {
    py_class: PyObject,
    root_rules: RootRules,
}
