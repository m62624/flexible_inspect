use super::traits::ValidationError;
use futures::Stream;

#[derive(Debug)]
pub struct ValidationErrorIterator {
    collection: Vec<Box<dyn ValidationError + Send>>,
}

impl ValidationErrorIterator {
    pub fn new(collection: Vec<Box<dyn ValidationError + Send>>) -> Self {
        Self { collection }
    }

    pub async fn async_into_iter(self) -> impl Stream<Item = Box<dyn ValidationError + Send>> {
        futures::stream::iter(self.collection.into_iter())
    }
}

impl Iterator for &mut ValidationErrorIterator {
    type Item = Box<dyn ValidationError + Send>;

    fn next(&mut self) -> Option<Self::Item> {
        self.collection.pop()
    }
}

impl AsRef<Vec<Box<dyn ValidationError + Send>>> for ValidationErrorIterator {
    fn as_ref(&self) -> &Vec<Box<dyn ValidationError + Send>> {
        &self.collection
    }
}

impl IntoIterator for ValidationErrorIterator {
    type Item = Box<dyn ValidationError + Send>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}
