use crate::export_lang::python_version::rules::traits::{PyRuleBase, PyRuleModifiers};
use log::error;
use pyo3::{exceptions, prelude::*, types};

pub trait PyCartridgeBase {
    type CartridgeType;
    fn to_rust(&mut self) -> Self::CartridgeType;
}
