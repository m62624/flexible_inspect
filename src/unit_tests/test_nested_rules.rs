use super::mock_obj;
use super::template_validator::TemplateValidator;
use super::*;

mod example_1 {
    use super::*;
    #[test]
    fn test_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = "[[AL_XL_191_KJ_OL]]";
            let class_error = mock_obj::make_obj(
                py,
                "custom error with value {number}",
                Some(vec![Rule::spawn(r"\[.+\]", MatchRequirement::MustBeFound)?
                    .extend_t(
                        py,
                        vec![
                            Rule::spawn(r"(?P<subrule>\[.+\])", MatchRequirement::MustBeFound)?
                                .extend_t(
                                    py,
                                    vec![Rule::spawn(
                                        r"(?P<number>\d+)",
                                        MatchRequirement::MustBeFound,
                                    )?],
                                )?,
                            Rule::spawn(
                                r".*",
                                MatchRequirement::MustBeFound,
                            )?,
                            Rule::spawn(
                                r".+",
                                MatchRequirement::MustBeFound,
                            )?,
                        ],
                    )?]),
            );
            let validator =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?;
            let values = validator.validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            if let Some(list) = values {
                dbg!(&list.downcast::<types::PyList>(py).unwrap());
            }
            Ok(())
        })
    }
}
