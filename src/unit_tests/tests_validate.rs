use super::captures::MultiCapture;
use super::*;

#[cfg(test)]
mod tests_actions_from_the_requirement {
    use super::*;

    #[cfg(test)]
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
                let mut next_step = false;
                validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut next_step,
                )?;
                assert_eq!(next_step, true);
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
                let mut next_step = false;
                validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut next_step,
                )?;
                assert_eq!(next_step, false);
                Ok(())
            })
        }

        /// MustBeFound, Captures - False, SubRules - True
        #[test]
        fn next_or_error_t_2() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                    .extend_t(py, vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound)?])
                    .unwrap();
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                let error = validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut false,
                )?;
                dbg!(&error);
                assert!(error.is_some());
                Ok(())
            })
        }
        /// MustBeFound, Captures - False, SubRules - False
        #[test]
        fn next_or_error_t_3() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                let error = validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut false,
                )?;
                dbg!(&error);
                assert!(error.is_some());
                Ok(())
            })
        }

        /// MustNotBeFound, Captures - True, SubRules - True
        #[test]
        fn next_or_error_t_4() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text [234 451] text text [text]";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound)?
                    .extend_t(py, vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound)?])?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                let mut next_step = false;
                validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut next_step,
                )?;
                assert_eq!(next_step, true);
                Ok(())
            })
        }

        /// MustNotBeFound, Captures - True, SubRules - False
        #[test]
        fn next_or_error_t_5() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text [234 451] text text [text]";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound)?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                let mut next_step = false;
                validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut next_step,
                )?;
                assert_eq!(next_step, false);
                Ok(())
            })
        }

        /// MustNotBeFound, Captures - False, SubRules - True
        #[test]
        fn next_or_error_t_6() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound)?
                    .extend_t(py, vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound)?])?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                let mut next_step = false;
                validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut next_step,
                )?;
                assert_eq!(next_step, false);
                Ok(())
            })
        }

        /// MustNotBeFound, Captures - True, SubRules - False
        #[test]
        fn next_or_error_t_7() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let text = "text text text text";
                let rule = Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustNotBefound)?;
                let class_template = mock_obj::make_obj(py, "GOTCHA : {data}", None);
                dbg!(&class_template);
                let captures = MultiCapture::find_captures(&rule, text)?;
                dbg!(&captures);
                let mut next_step = false;
                validate::actions_from_the_requirement::next_or_error(
                    py,
                    &class_template,
                    &rule,
                    captures,
                    &mut next_step,
                )?;
                assert_eq!(next_step, false);
                Ok(())
            })
        }
    }
}
