pub mod actions_from_the_requirement;
use super::template_validator::TemplateValidator;
use super::*;

#[pymethods]
impl TemplateValidator {
    /// Проверка и конвертация байтов в строку (`UTF-8`)
    fn single_validate(&self, text: &types::PyBytes) -> PyResult<()> {
        let mut errors: Vec<PyObject> = Vec::new();
        let text = Self::bytes_to_string_utf8(text.as_bytes())?;
        self.get_exceptions()
            .iter()
            .map(|class_py| {
                //====
                Ok(())
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}
