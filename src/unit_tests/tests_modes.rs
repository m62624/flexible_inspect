use super::mock_obj;
use super::template_validator::TemplateValidator;
use super::*;

mod mode_all_rules_for_all_matches {
    use super::*;

    #[test]
    fn runner_t_0() -> PyResult<()> {
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
                            Rule::spawn(r"(?P<subrule>\[.+\])", MatchRequirement::MustBeFound)?,
                            Rule::spawn(r".*", MatchRequirement::MustBeFound)?,
                            Rule::spawn(r".+", MatchRequirement::MustBeFound)?,
                        ],
                    )?]),
            );
            let values =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?
                    .validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), false);
            Ok(())
        })
    }

    #[test]
    fn runner_t_1() -> PyResult<()> {
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
                                    vec![
                                        Rule::spawn(
                                            r"(?P<number>\d+)",
                                            MatchRequirement::MustNotBeFound,
                                        )?,
                                        Rule::spawn(r"BOBA", MatchRequirement::MustNotBeFound)?,
                                    ],
                                )?,
                            Rule::spawn(r"ABOBA", MatchRequirement::MustNotBeFound)?,
                            Rule::spawn(r".*", MatchRequirement::MustBeFound)?,
                            Rule::spawn(r".+", MatchRequirement::MustBeFound)?,
                        ],
                    )?]),
            );
            let values =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?
                    .validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), true);
            Ok(())
        })
    }
}

mod mode_all_rules_for_at_least_one_match {
    use super::*;

    #[test]
    fn runner_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = "BOBA [[AL_XL_191_KJ_OL]] QLWOE [ABOBA] [BOBA]";
            let class_error = mock_obj::make_obj(
                py,
                "custom error with value {number}",
                Some(vec![Rule::spawn(
                    r"\[[^\]]+\]",
                    MatchRequirement::MustBeFound,
                )?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"(?P<subrule>\[.+\])", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"ABOBA", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d+", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r".*", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r".+", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_all_rules_for_at_least_one_match()]),
            );
            let values =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?
                    .validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), false);
            Ok(())
        })
    }

    #[test]
    fn runner_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = "BOBA [[AL_XL_191_KJ_OL]] QLWOE [BOBA]";
            let class_error = mock_obj::make_obj(
                py,
                "custom error with value {number}",
                Some(vec![Rule::spawn(
                    r"\[[^\]]+\]",
                    MatchRequirement::MustBeFound,
                )?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"(?P<subrule>\[.+\])", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"ABOBA", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d+", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r".*", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r".+", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_all_rules_for_at_least_one_match()]),
            );
            let values =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?
                    .validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), true);
            Ok(())
        })
    }
}

mod mode_at_least_one_rule_for_all_matches {
    use super::*;

    #[test]
    fn runner_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = " ABOBA [123] [23] [1331] QWEQ";
            let class_error = mock_obj::make_obj(
                py,
                "custom error with value {number}",
                Some(vec![Rule::spawn(
                    r"\[[^\]]+\]",
                    MatchRequirement::MustBeFound,
                )?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"ABOBA", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d)", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"LP", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()]),
            );
            let validator =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?;
            let values = validator.validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), false);
            Ok(())
        })
    }

    #[test]
    fn runner_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = " ABOBA [123] [23] [slqp] QWEQ";
            let class_error = mock_obj::make_obj(
                py,
                "custom error with value {number}",
                Some(vec![Rule::spawn(
                    r"\[[^\]]+\]",
                    MatchRequirement::MustBeFound,
                )?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"ABOBA", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d)", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"LP", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()]),
            );
            let validator =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?;
            let values = validator.validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), true);
            Ok(())
        })
    }
}

mod mode_at_least_one_rule_for_at_least_one_match {
    use super::*;

    #[test]
    fn runner_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = "LP [123] [23] [slqp] QWEQ";
            let class_error = mock_obj::make_obj(
                py,
                "custom error with value {number}",
                Some(vec![Rule::spawn(
                    r"\[[^\]]+\]",
                    MatchRequirement::MustBeFound,
                )?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"ABOBA", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d)", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"LP", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()]),
            );
            let validator =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?;
            let values = validator.validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), false);
            Ok(())
        })
    }

    #[test]
    fn runner_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = "QWEQ";
            let class_error = mock_obj::make_obj(
                py,
                "custom error with value {number}",
                Some(vec![Rule::spawn(
                    r"\[[^\]]+\]",
                    MatchRequirement::MustBeFound,
                )?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"ABOBA", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d)", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"LP", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()]),
            );
            let validator =
                TemplateValidator::new(py, types::PyList::new(py, [class_error]).into_py(py))?;
            let values = validator.validate(py, types::PyBytes::new(py, text.as_bytes()))?;
            assert_eq!(values.is_some(), true);
            Ok(())
        })
    }
}
