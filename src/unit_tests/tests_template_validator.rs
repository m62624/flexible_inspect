use super::*;

mod fn_new {
    use super::*;

    #[test]
    fn new_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let rule1 = Rule::spawn(r"(?P<number>\d+)", MatchRequirement::MustNotBeFound)?;
            let class_py = mock_obj::make_obj(py, "test", Some(vec![rule1]));

            let template_val = template_validator::TemplateValidator::new(
                py,
                types::PyList::new(py, [class_py]).into_py(py),
            )?;
            dbg!(&template_val);
            Ok(())
        })
    }
}
mod fn_validate {
    use super::*;
    #[test]
    fn validate_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let rule1 = Rule::spawn(r"(?P<number>\d+)", MatchRequirement::MustNotBeFound)?;
            let class_py = mock_obj::make_obj(py, "test", Some(vec![rule1]));

            let template_val = template_validator::TemplateValidator::new(
                py,
                types::PyList::new(py, [class_py]).into_py(py),
            )?;
            let text_bytes = types::PyBytes::new(py, "test 123".as_bytes());
            dbg!(&template_val);
            assert_eq!(template_val.validate(py, text_bytes)?.is_some(), true);
            Ok(())
        })
    }
}

mod fn_async_validate {
    use super::*;
    use crate::{cartridge::CartridgeWrapper, rule::next::NextStep};

    #[test]
    fn async_validate_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let rule1 = Rule::spawn(r"(?P<number>\d+)", MatchRequirement::MustNotBeFound)?;
            let class_py = mock_obj::make_obj(py, "test", Some(vec![rule1]));
            let text = "test 123".to_string();

            async_std::task::block_on(async {
                assert_eq!(
                    matches!(
                        CartridgeWrapper::new(py, class_py)?
                            .async_run(Arc::new(text))
                            .await,
                        NextStep::Error(_)
                    ),
                    true
                );
                Ok(())
            })
        })
    }
}
