use pyo3::pyclass;

mod tests_cartridges;
mod tests_rules;

#[pyclass]
#[derive(Clone, Debug)]
struct FakeObject;
