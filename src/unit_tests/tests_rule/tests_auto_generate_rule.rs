use super::*;

/// Проверка генераций правила c помощью grex
#[test]
fn auto_generate_t_0() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let texts = types::PyList::new(
            py,
            ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"],
        );
        assert_eq!(
            Rule::auto_generate(MatchRequirement::MustBeFound, texts)?.as_ref(),
            "^(?:1[01]|1|[2-9])$"
        );

        Ok(())
    })
}

/// Проверка генераций правила c помощью grex
#[test]
fn auto_generate_t_1() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let texts = types::PyList::new(py, ["bob", "aboba", "b", "ba"]);
        assert_eq!(
            Rule::auto_generate(MatchRequirement::MustBeFound, texts)?.as_ref(),
            "^(?:aboba|b(?:(?:ob|a))?)$"
        );
        Ok(())
    })
}
 