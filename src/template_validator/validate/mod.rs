mod bytes_convert;
mod next_step;
mod runners;

use crate::rule::{MatchRequirement, RegexRaw};

use self::runners::single_work::step_by_step_in_the_class;

//=====================
use super::rule::Rule;
use super::*;
use bytes_convert::bytes_to_string_utf8;
use pyo3::types::PyList;
#[pymethods]
impl TemplateValidator {
    fn validate_single_sync(&self, py: Python, text: &types::PyBytes) -> PyResult<()> {

        let mut errors = Vec::new();
        let text = bytes_to_string_utf8(text.as_bytes())?;
        self.exceptions
            .iter()
            .map(|exception_container| {
                dbg!(&exception_container);
                runners::single_work::step_by_step_in_the_class(
                    py,
                    &mut errors,
                    exception_container,
                    &text,
                )
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}

// #[test]
// fn validate_step() {
//     Python::with_gil(|py| {
//         let rules = PyList::new(py, [rust_rule(py, "aboba", MatchRequirement::MustBeFound)]);
//         let mut errors = Vec::new();
//         step_by_step_in_the_class(py, &mut errors, exception_class, text);
//     });
// }

// fn extend_rust_rule(rules: &[Rule]) -> PyObject {
//     let pyl = PyList::new(py, elements);
// }
fn rust_rule(py: Python, str: &str, matchrq: MatchRequirement) -> PyObject {
    Rule::new(str.to_string(), matchrq).unwrap().to_object(py)
}

impl ToPyObject for Rule {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let rule_dict = types::PyDict::new(py);

        if let Some(str_raw) = self.get_op_str_raw() {
            match str_raw {
                RegexRaw::DefaultR(value) => {
                    rule_dict.set_item("str_raw", value.to_object(py)).unwrap();
                    rule_dict
                        .set_item("type", "DefaultR".to_object(py))
                        .unwrap();
                }
                RegexRaw::FancyR(value) => {
                    rule_dict.set_item("str_raw", value.to_object(py)).unwrap();
                    rule_dict.set_item("type", "FancyR".to_object(py)).unwrap();
                }
            }
        }

        if let Some(requirement) = &self.get_op_requirement() {
            match requirement {
                MatchRequirement::MustBeFound => {
                    rule_dict
                        .set_item("requirement", "MustBeFound".to_object(py))
                        .unwrap();
                }
                MatchRequirement::MustNotBefound => {
                    rule_dict
                        .set_item("requirement", "MustNotBefound".to_object(py))
                        .unwrap();
                }
            }
        }

        if let Some(subrules) = &self.get_op_subrules() {
            let subrules_list = PyList::empty(py);
            for subrule in subrules {
                subrules_list.append(subrule.to_object(py)).unwrap();
            }
            rule_dict.set_item("subrules", subrules_list).unwrap();
        }

        rule_dict.to_object(py)
    }
}
