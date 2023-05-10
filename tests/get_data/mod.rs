use pyo3::prelude::*;
use pyo3::types;
use pyo3::PyResult;
use pystval::It;
use pystval::TemplateValidator;
use pystval::{MESSAGE_WITH_EXTRA_FROM_CLASS_PY, RULES_FROM_CLASS_PY};

pub const LINK_HTML_1: &'static str = "https://test-cdn.yourbandy.com/profile_templates/b7888914-6356-4904-8950-774e3057e034_profile_template_28.html";

// Вспомогательная  функция по создание  Py объектов
// По большей части, является пустышкой с regex и message
pub fn create_obj(rules: Option<&[(&str, It)]>, msg: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| -> PyResult<PyObject> {
        let dict = types::PyDict::new(py);
        if let Some(rules) = rules {
            for (key, value) in rules.iter() {
                dict.set_item(key, Py::new(py, value.to_owned()).unwrap())?;
            }
        }
        let class = types::PyType::new::<TemplateValidator>(py);
        class.setattr(RULES_FROM_CLASS_PY, dict)?;
        class.setattr(
            MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
            types::PyString::new(py, format!("{}", msg).as_str()),
        )?;
        Ok(class.to_object(py))
    })
}

pub fn list_for_pystval(py: Python, obj: Vec<Py<PyAny>>) -> &types::PyList {
    types::PyList::new(py, obj.iter())
}

// Получаем содержимое
pub async fn get_bytes_from_html() -> Result<Box<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(LINK_HTML_1).send().await;
    let html = response?.text().await?;
    Ok(Box::new(html))
}
