use std::{error::Error, fmt};
#[derive(Debug, Clone)]
pub struct PystvalError {
    id: i64,
    msg: String,
}

impl Error for PystvalError {}

impl PystvalError {
    pub fn new(id: i64, msg: String) -> Self {
        Self { id, msg }
    }

    pub fn get_code(&self) -> i64 {
        self.id
    }
}

impl fmt::Display for PystvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| ERROR CODE {id} |\n{msg}",
            id = self.id,
            msg = self.msg
        )
    }
}
