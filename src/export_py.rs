use crate::*;
#[pymodule]
fn pystval(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<CustomValidator>()?;
    Ok(())
}
