#[cfg(any(feature = "serde", feature = "wasm"))]
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

// We implement our own error for each cartridge

/// This Error stores data from the `cartridges`, an `error message` with data, and an `error code`
#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PystvalError {
    pub(crate) id: i64,
    pub(crate) msg: String,
}

impl Error for PystvalError {}

impl PystvalError {
    pub fn new(id: i64, msg: String) -> Self {
        Self { id, msg }
    }

    /// Returns the error code
    pub fn get_code(&self) -> i64 {
        self.id
    }

    /// Returns the error message
    pub fn get_message(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for PystvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(ERROR CODE |{id}|) {msg}", id = self.id, msg = self.msg)
    }
}
