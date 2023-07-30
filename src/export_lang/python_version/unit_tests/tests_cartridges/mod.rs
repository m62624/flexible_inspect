use pyo3::pyclass;

mod tests_cartridges_bytes;
mod tests_cartridges_str;

#[pyclass]
#[derive(Clone, Debug)]
struct FakeObject;
