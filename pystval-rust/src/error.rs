pub trait ValidationError {
    fn get_code(&self) -> i32;
    fn get_message(&self) -> &str;
}

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

pub struct ValidationErrorIterator {
    collection: Vec<Box<dyn ValidationError>>,
}

impl ValidationErrorIterator {
    pub fn new(collection: Vec<Box<dyn ValidationError>>) -> Self {
        Self { collection }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn ValidationError>> {
        self.collection.iter()
    }
}

impl IntoIterator for ValidationErrorIterator {
    type Item = Box<dyn ValidationError>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}
