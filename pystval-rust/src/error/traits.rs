use super::*;

pub trait ValidationError {
    fn get_code(&self) -> i32;
    fn get_message(&self) -> &str;
}