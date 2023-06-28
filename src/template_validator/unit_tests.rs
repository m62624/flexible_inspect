use super::*;
use crate::{rule::Rule, template_validator::TemplateValidator};
use pyo3::types::PyList;

fn make_obj(py: Python, message: &str, rules: Vec<Rule>) -> PyObject {
    let rules = types::PyList::new(py, rules.into_iter().map(|r| r.into_py(py)));
    let obj = types::PyType::new::<Rule>(py);
    obj.setattr(
        MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
        types::PyString::new(py, format!("{}", message).as_str()),
    )
    .unwrap();
    obj.setattr(RULES_FROM_CLASS_PY, rules).unwrap();
    obj.into()
}

#[cfg(test)]
mod TemplateValidator_contructor {

    use super::*;
    #[test]
    fn constructor_test_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let template_validator = TemplateValidator::new(
                py,
                PyList::new(
                    py,
                    vec![make_obj(
                    py,
                    "",
                    vec![Rule::new(r"\d".to_string(), rule::MatchRequirement::MustBeFound).unwrap()],
                )],
                )
                .to_object(py),
            );
            dbg!(&template_validator);
            Ok(())
        })
    }
}
