use std::fmt::Debug;
pub trait ValidationError: Send + Sync + Debug {
    fn get_code(&self) -> i32;
    fn get_message(&self) -> &str;
}
