mod cartridges;
mod error;
mod template_validator;
mod rules;
use pyo3::prelude::*;
use pystval::prelude::*;
use rules::{rule_bytes::PyRuleBytes, rule_str::PyRule};
use cartridges::{cartridges_bytes::PyCartridgeBytes, cartridges_str::PyCartridge};