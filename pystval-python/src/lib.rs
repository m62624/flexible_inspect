mod cartridges;
mod rules;
use pyo3::prelude::*;
use pystval::prelude::*;
use rules::{rule_bytes::PyRuleBytes, rule_str::PyRule};
