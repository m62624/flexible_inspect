use super::mock_obj;
use super::rule::{MatchRequirement, Rule};
use super::unit_tests::mock_obj::CustomClassError;
use super::*;
#[cfg(test)]
mod fn_new {
    use super::*;

    #[test]
    fn constructor_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let rule_1 = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound)?.extend_t(
                py,
                vec![
                    Rule::spawn(r"\[.+\]", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"(?P<data>\w)", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?,
                ],
            )?;
            let rule_2 = Rule::spawn(r"[^ABC]", MatchRequirement::MustBeFound)?;
            let rule_3 = Rule::spawn(r"[^123]", MatchRequirement::MustNotBefound)?;
            let class_py =
                mock_obj::make_obj(py, "it found: {data}", Some(vec![rule_1, rule_2, rule_3]));
            dbg!(excpetion_container::ExceptionContainer::new(py, class_py)?);
            Ok(())
        })
    }

    #[test]
    #[should_panic]
    fn constructor_e_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let obj = types::PyType::new::<CustomClassError>(py);
            obj.setattr(
                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                types::PyString::new(py, format!("__").as_str()),
            )
            .unwrap();
            obj.setattr(
                RULES_FROM_CLASS_PY,
                types::PyType::new::<CustomClassError>(py),
            )
            .unwrap();
            dbg!(excpetion_container::ExceptionContainer::new(py, obj.into_py(py)).unwrap());
        });
    }

    #[test]
    #[should_panic]
    fn constructor_e_1() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let obj = types::PyType::new::<CustomClassError>(py);
            dbg!(excpetion_container::ExceptionContainer::new(py, obj.into_py(py)).unwrap());
        });
    }
}
