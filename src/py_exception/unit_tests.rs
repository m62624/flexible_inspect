#[cfg(test)]
use super::*;
#[cfg(test)]
use crate::rule::MatchRequirement;
#[cfg(test)]
use crate::rule::Rule;

#[cfg(test)]
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
mod fn_get_extra_from_class {
    use super::*;

    #[test]
    fn get_extra_from_class_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let obj = make_obj(
                py,
                "x: {x-1}, y: {y-1}",
                vec![Rule::spawn(r"\d", MatchRequirement::MustBeFound)?],
            );
            let extra =
                extra_collection::get_extra_from_class(py, &obj, MESSAGE_WITH_EXTRA_FROM_CLASS_PY)?;
            dbg!(&extra);
            assert!(!extra.is_empty());
            Ok(())
        })
    }
}

#[cfg(test)]
mod fn_filling_extra {

    use super::*;
    use crate::template_validator::exception_container;

    #[test]
    fn filling_extra_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let text = "x-123,x-456,y-789,y-101112";
            let obj = make_obj(
                py,
                "x: {X}, y: {Y}",
                vec![
                    Rule::spawn(r"(?P<X>x-\d+)", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"(?P<Y>y-\d+)", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"(?P<Y>y-(?=\d+))", MatchRequirement::MustBeFound)?,
                ],
            );
            let extra =
                extra_collection::get_extra_from_class(py, &obj, MESSAGE_WITH_EXTRA_FROM_CLASS_PY)?;
            let mut default_rules = Vec::new();
            exception_container::ExceptionContainer::get_all_rules_from_class(
                obj.downcast::<types::PyType>(py)?,
                &mut default_rules,
                &mut vec![],
            )?;
            let extra_x = extra_collection::filling_extra(
                &extra,
                captures::MultiCapture::find_captures(&default_rules[0], &text)?,
            );
            let extra_y = extra_collection::filling_extra(
                &extra,
                captures::MultiCapture::find_captures(&default_rules[1], &text)?,
            );
            assert!(extra_x.is_some());
            assert!(extra_y.is_some());
            dbg!(extra_x);
            dbg!(extra_y);
            Ok(())
        })
    }
}
