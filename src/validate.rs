use pyo3::exceptions;

use super::*;
impl TemplateValidator {
    pub fn core_validate(&self, text: String) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            for match_idx in self.selected_simple_rules.matches(&text).iter() {
                let rule = &self.selected_simple_rules.patterns()[match_idx];
                if let Some(rule_status) = self.all_simple_rules.get(rule) {
                    just_look_at_this_simple(py, self, rule, rule_status, &text)?;
                }
            }
            Ok(())
        })
    }
}
fn just_look_at_this_simple(
    py: Python,
    slf: &TemplateValidator,
    regex: &String,
    rule_status: &RuleStatus,
    text: &str,
) -> PyResult<()> {
    //=========================================================================
    let mut extra_values: HashMap<String, String> = HashMap::new();
    let obj = slf
        .python_classes
        .get(&rule_status.id)
        .unwrap()
        .to_object(py);
    let obj_for_extra = obj.downcast::<types::PyType>(py).unwrap();
    //=========================================================================
    if let Some(extra_names) =
        make_errors::extra_from_class(obj_for_extra, MESSAGE_WITH_EXTRA_FROM_CLASS_PY).unwrap()
    {
        for capture in check_convert::convert::string_to_default_regex(regex).captures_iter(text) {
            for name in &extra_names {
                match capture.name(&name) {
                    // заполняем значение, если оно есть
                    Some(value) => {
                        extra_values.insert(name.to_string(), value.as_str().to_string());
                    }
                    // если нет, то в template вместо **extra выйдет "___"
                    None => {
                        extra_values.insert(name.to_string(), "___".to_string());
                    }
                }
            }
        }
    }
    match rule_status.status {
        It::MustBeFoundHere => Ok(()),
        It::NotToBeFoundHere => {
            if extra_values.is_empty() {
                make_errors::throw_error(&obj, None)
            } else {
                make_errors::throw_error(&obj, Some(extra_values))
            }
        }
    }
    // получаем каждое совпадение
}

/*

let mut value_for_extra: Option<HashMap<String, String>> = None;
if let Some(extra) = make_errors::extra_from_class(
    slf.python_classes
        .get(&rule_status.id)
        .unwrap()
        .to_object(py)
        .downcast::<types::PyType>(py)
        .unwrap(),
    EXTRA_FOR_ERROR_PY,
)
.unwrap()
{
    let mut values: HashMap<String, String> = HashMap::new();
    for capture in check_convert::convert::string_to_default_regex(regex).captures_iter(text) {
        // проверяем есть ли найденная группа из regex для **extra
        for name in extra {
            match capture.name(&name) {
                // заполняем значение, если оно есть
                Some(value) => {
                    values.insert(name.to_string(), value.as_str().to_string());
                }
                // если нет, то в template вместо **extra выйдет "___"
                None => {
                    values.insert(name.to_string(), "___".to_string());
                }
            }
        }
        value_for_extra = Some(values);
    } */

// for name in extra {
//     match capture.name(name) {
//         // заполняем значение, если оно есть
//         Some(value) => {
//             result.insert(name.to_string(), value.as_str().to_string());
//         }
//         // если нет, то в template вместо **extra выйдет "___"
//         None => {
//             result.insert(name.to_string(), "___".to_string());
//         }
//     }
// }

// let mut match_found = false;
// let mut extra: HashMap<String, String> = HashMap::new();
// for cap in check_convert::convert::string_to_default_regex(regex).captures_iter(text) {
//     match_found = true;
// }
// check_convert::convert::string_to_default_regex(regex)
//     .captures_iter(text)
//     .map(|cap| {})
//     .for_each(drop);

// fn if_found(
//     py: Python,
//     slf: &TemplateValidator,
//     rule_status: &RuleStatus,
//     match_found: bool,
//     extra_hm: Option<HashMap<String, String>>,
// ) -> PyResult<()> {
//     let obj_for_extra = || {
//         slf.python_classes
//             .get(&rule_status.id)
//             .unwrap()
//             .to_object(py)
//     };
//     // let extra = || {
//     //     make_errors::extra_from_class(
//     //         obj_for_extra().downcast::<types::PyType>(py).unwrap(),
//     //         EXTRA_FOR_ERROR_PY,
//     //     )
//     //     .unwrap()
//     // };

//     match (&rule_status.status, match_found) {
//         (It::MustBeFoundHere, true) => return Ok(()),
//         (It::MustBeFoundHere, false) => {
//             return make_errors::throw_error(&obj_for_extra(), extra_hm);
//         }
//         (It::NotToBeFoundHere, true) => {
//             return make_errors::throw_error(&obj_for_extra(), extra_hm);
//         }
//         (It::NotToBeFoundHere, false) => return Ok(()),
//     }
// }
