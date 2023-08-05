use super::traits::ValidationError;
use futures::Stream;

pub struct ValidationErrorIterator {
    collection: Vec<Box<dyn ValidationError>>,
}

impl ValidationErrorIterator {
    pub fn new(collection: Vec<Box<dyn ValidationError>>) -> Self {
        Self { collection }
    }

    pub async fn async_into_iter(self) -> impl Stream<Item = Box<dyn ValidationError>> {
        futures::stream::iter(self.collection.into_iter())
    }
}

impl Iterator for &mut ValidationErrorIterator {
    type Item = Box<dyn ValidationError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.collection.pop()
    }
}

impl AsRef<Vec<Box<dyn ValidationError>>> for ValidationErrorIterator {
    fn as_ref(&self) -> &Vec<Box<dyn ValidationError>> {
        &self.collection
    }
}

impl IntoIterator for ValidationErrorIterator {
    type Item = Box<dyn ValidationError>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}
