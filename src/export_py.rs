use crate::*;
#[pymodule]
fn pystval(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Validator>()?;
    // m.add_function(wrap_pyfunction!(init_validator::validate, m)?)?;
    Ok(())
}
