use super::*;

mod fn_export_base_error {
    use super::*;

    /// Проверка экспорта базового шаблона ошибки
    #[test]
    fn export_base_error_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            PyModule::from_code(py, &base_error::export_base_error(), "", MODULE_NAME)?;
            Ok(())
        })
    }
}
