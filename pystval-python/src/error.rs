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
    pub fn next(&mut self) -> Option<PyBaseValidationError> {
        self.0.pop().map(|error| PyBaseValidationError {
            code: error.get_code(),
            message: error.get_message().to_owned(),
        })
    }

    pub fn for_each(&self, py: Python, callback: PyObject) -> PyResult<Vec<PyObject>> {
        self.0.iter().try_fold(Vec::new(), |mut acc, item| {
            let result = callback.call1(py, (item.get_code(), item.get_message()))?;
            acc.push(result);
            Ok(acc)
        })
    }

    pub fn if_error(&self, py: Python, callback: PyObject) -> PyResult<Vec<PyObject>> {
        if self.0.is_empty() {
            Ok(Vec::new())
        } else {
            self.for_each(py, callback)
        }
    }

    pub fn if_ok(&self, py: Python, callback: PyObject) -> PyResult<PyObject> {
        if self.0.is_empty() {
            Ok(callback.call0(py)?)
        } else {
            Ok(py.None())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
