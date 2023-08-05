pub mod async_error;
pub mod sync_error;
pub mod traits;
use self::traits::ValidationError;

pub struct BaseValidationError {
    code: i32,
    message: String,
}

impl BaseValidationError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }
}

impl ValidationError for BaseValidationError {
    fn get_code(&self) -> i32 {
        self.code
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}
