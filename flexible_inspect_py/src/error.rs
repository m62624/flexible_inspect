use super::*;

#[pyclass(name = "ValidationError")]
#[derive(Clone)]
pub struct PyBaseValidationError {
    code: i32,
    message: String,
}

#[pymethods]
impl PyBaseValidationError {
    pub fn get_code(&self) -> i32 {
        self.code
    }

    pub fn get_message(&self) -> String {
        self.message.to_owned()
    }
}

#[pyclass(name = "ValidationErrorIterator")]
pub struct PyValidationErrorIterator(pub(crate) Vec<Box<dyn ValidationError + Send>>);

#[pymethods]
impl PyValidationErrorIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyBaseValidationError> {
        slf.0.pop().map(|error| PyBaseValidationError {
            code: error.get_code(),
            message: error.get_message().to_owned(),
        })
    }

    pub fn __len__(slf: PyRef<'_, Self>) -> usize {
        slf.0.len()
    }

    /// Returns `True` if the error code is in the collection.
    pub fn __contains__(slf: PyRef<'_, Self>, code: i32) -> bool {
        slf.0.iter().any(|error| error.get_code() == code)
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyResult<Py<PyValidationErrorIterator>> {
        Ok(slf.into())
    }

    fn __anext__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<PyBaseValidationError>> {
        Ok(slf.0.pop().map(|error| PyBaseValidationError {
            code: error.get_code(),
            message: error.get_message().to_owned(),
        }))
    }
}

impl PyValidationErrorIterator {
    pub fn new(collection: Vec<Box<dyn ValidationError + Send>>) -> Self {
        Self(collection)
    }
}
