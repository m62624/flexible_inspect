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
