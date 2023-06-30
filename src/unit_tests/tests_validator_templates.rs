use super::captures::MultiCapture;
use super::*;

mod tests_actions_from_the_requirement {
    use super::*;

    mod fn_next_or_error {
        use super::*;

        /// MustBeFound, Captures - True, SubRules - True
        #[test]
        fn next_or_error_t_0() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text [234 451] text text [text]";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                    .extend_t(py, vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound)?])?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                assert_eq!(
                    validator_templates::actions_from_the_requirement::next_or_error(
                        py,
                        &class_template,
                        &rule,
                        &captures,
                    )?,
                    true
                );
                Ok(())
            })
        }

        /// MustBeFound, Captures - True, SubRules - False
        #[test]
        fn next_or_error_t_1() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text [234 451] text text [text]";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                assert_eq!(
                    validator_templates::actions_from_the_requirement::next_or_error(
                        py,
                        &class_template,
                        &rule,
                        &captures,
                    )?,
                    false
                );
                Ok(())
            })
        }

        /// MustBeFound, Captures - False, SubRules - True
        #[test]
        #[should_panic]
        fn next_or_error_e_0() {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)
                    .unwrap()
                    .extend_t(
                        py,
                        vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound).unwrap()],
                    )
                    .unwrap();
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text).unwrap();
                dbg!(&captures);
                validator_templates::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    &captures,
                )
                .unwrap();
            });
        }

        /// MustBeFound, Captures - False, SubRules - False
        #[test]
        #[should_panic]
        fn next_or_error_e_1() {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound).unwrap();
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text).unwrap();
                dbg!(&captures);
                validator_templates::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    &captures,
                )
                .unwrap();
            });
        }

        /// MustNotBeFound, Captures - True, SubRules - True
        #[test]
        fn next_or_error_t_2() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text [234 451] text text [text]";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound)?
                    .extend_t(py, vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound)?])?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                assert_eq!(
                    validator_templates::actions_from_the_requirement::next_or_error(
                        py,
                        &class_template,
                        &rule,
                        &captures,
                    )?,
                    true
                );
                Ok(())
            })
        }

        /// MustNotBeFound, Captures - True, SubRules - False
        #[test]
        #[should_panic]
        fn next_or_error_e_2() {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| {
                let text = "text text [234 451] text text [text]";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound).unwrap();
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text).unwrap();
                dbg!(&captures);
                validator_templates::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    &captures,
                )
                .unwrap();
            });
        }

        /// MustNotBeFound, Captures - False, SubRules - True
        #[test]
        fn next_or_error_t_3() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound)?
                    .extend_t(py, vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound)?])?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                assert_eq!(
                    validator_templates::actions_from_the_requirement::next_or_error(
                        py,
                        &class_template,
                        &rule,
                        &captures,
                    )?,
                    false
                );
                Ok(())
            })
        }

        /// MustNotBeFound, Captures - False, SubRules - False
        #[test]
        fn next_or_error_t_4() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound)?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                assert_eq!(
                    validator_templates::actions_from_the_requirement::next_or_error(
                        py,
                        &class_template,
                        &rule,
                        &captures,
                    )?,
                    false
                );
                Ok(())
            })
        }
    }
}
