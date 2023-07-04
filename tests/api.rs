use std::fs;

use pyo3::prelude::*;
#[test]
fn run_py_code() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let py_app = fs::read_to_string("tests/polygon/py_code.py")?;
        
        // dbg!(&py_app);
        // PyModule::from_code(py, "", &py_app, "");
        // let x = PyModule::from_code(py, "", "tests/polygon/py_code.py", "pystval")?;
        dbg!(py.run(&py_app, None, None));
        Ok(())
    })
    //
}
