use super::mock_obj;
use super::rule::{MatchRequirement, Rule};
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

}
