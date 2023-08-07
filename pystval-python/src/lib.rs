mod cartridges;
mod rules;
use pyo3::exceptions::{self, PyException};
use pyo3::prelude::*;
use pyo3::types;
use pystval::prelude::*;
use rules::{rule_bytes::PyRuleBytes, rule_str::PyRule};
