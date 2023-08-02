use super::*;
use std::{error::Error, fmt};

#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
#[derive(Debug, Clone)]
pub struct PystvalError {
    pub(crate) id: i64,
    pub(crate) msg: String,
}

impl Error for PystvalError {}

impl PystvalError {
    pub fn new(id: i64, msg: String) -> Self {
        Self { id, msg }
    }

    pub fn get_code(&self) -> i64 {
        self.id
    }

    pub fn get_message(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for PystvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(ERROR CODE |{id}|) {msg}", id = self.id, msg = self.msg)
    }
}
