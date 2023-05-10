use pyo3::{PyResult, Python};
use pystval::{self, It};
mod get_data;
use crate::get_data::list_for_pystval;
use get_data::{create_obj, get_bytes_from_html};
use tokio;

#[tokio::test]
async fn check_image() -> PyResult<()> {
    let text = get_bytes_from_html().await.unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let rules = [(r#"<im id="avatar"\s?.+?\s?>"#, It::MustBeFoundHere)];
        let message = "`savatar` not found";
        let pstvl = pystval::TemplateValidator::__new__(
            list_for_pystval(py, vec![create_obj(Some(&rules), message).unwrap()]).into(),
        )
        .unwrap();
        if let Err(err) = pstvl.core_validate(text.to_string()) {
            println!("!! валидация не пройдена : {}", err);
        } else {
            println!(":: валидация пройдена");
        }
    });
    Ok(())
}
