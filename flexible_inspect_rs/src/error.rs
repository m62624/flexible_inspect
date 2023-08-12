use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ValidationError {
    code: i32,
    message: String,
}

impl ValidationError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn into_inner(self) -> (i32, String) {
        (self.code, self.message)
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.code, self.message)
    }
}

impl Error for ValidationError {}
