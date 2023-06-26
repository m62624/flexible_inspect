use super::*;
pub fn bytes_to_string_utf8(bytes: &[u8]) -> PyResult<String> {
    match String::from_utf8(bytes.into()) {
        Ok(result) => Ok(result),
        Err(_) => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
            "{:#?} is not a valid UTF-8 string",
            bytes
        ))),
    }
}
