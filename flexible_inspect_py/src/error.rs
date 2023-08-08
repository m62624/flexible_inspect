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
pub struct PyValidationErrorIterator(Vec<Box<dyn ValidationError + Send>>);

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

    // pub fn for_each(&self, py: Python, callback: PyObject) -> PyResult<Vec<PyObject>> {
    //     self.0.iter().try_fold(Vec::new(), |mut acc, item| {
    //         let result = callback.call1(py, (item.get_code(), item.get_message()))?;
    //         acc.push(result);
    //         Ok(acc)
    //     })
    // }

    // pub fn for_each_with_reserved_params(&self, py: Python, callback: PyObject) -> PyResult<Vec<PyObject>> {
    //     self.0.iter().try_fold(Vec::new(), |mut acc, item| {
    //         let result = callback.call1(py, (item.get_code(), item.get_message()))?;
    //         acc.push(result);
    //         Ok(acc)
    //     })
    // }

    // pub fn if_error_with_reserved_params(&self, py: Python, callback: PyObject) -> PyResult<Vec<PyObject>> {
    //     if self.0.is_empty() {
    //         Ok(Vec::new())
    //     } else {
    //         self.for_each_with_reserved_params(py, callback)
    //     }
    // }

    // pub fn if_ok(&self, py: Python, callback: PyObject) -> PyResult<PyObject> {
    //     if self.0.is_empty() {
    //         Ok(callback.call0(py)?)
    //     } else {
    //         Ok(py.None())
    //     }
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.0.is_empty()
    // }

    // pub fn len(&self) -> usize {
    //     self.0.len()
    // }
}

impl PyValidationErrorIterator {
    pub fn new(collection: Vec<Box<dyn ValidationError + Send>>) -> Self {
        Self(collection)
    }
}
