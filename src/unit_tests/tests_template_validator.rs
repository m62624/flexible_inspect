use super::*;

mod fn_new {
    use super::*;

    #[test]
    fn new_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let rule1 = Rule::spawn(r"(?P<number>\d+)", MatchRequirement::MustNotBefound)?;
            let class_py = mock_obj::make_obj(py, "test", Some(vec![rule1]));

            let template_val = template_validator::TemplateValidator::new(
                py,
                types::PyList::new(py, [class_py]).into_py(py),
            )?;
            let text_bytes = types::PyBytes::new(py, "test 123".as_bytes());
            dbg!(&template_val);
            template_val.validate(py, text_bytes)?;
            Ok(())
        })
    }
}
