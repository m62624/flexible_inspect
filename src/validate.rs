use pyo3::exceptions;

use super::*;
impl TemplateValidator {
    pub fn core_validate(&self, text: String) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            for match_idx in self.selected_simple_rules.matches(&text).iter() {
                let rule = &self.selected_simple_rules.patterns()[match_idx];
                just_look_at_this_simple(
                    py,
                    self,
                    rule,
                    self.all_simple_rules.get(rule).unwrap(),
                    &text,
                )?;
            }
            for (rule, rule_status) in self.all_simple_rules.iter() {
                just_look_at_this_simple(py, self, rule, rule_status, &text)?;
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
    let mut flag = false;
    let mut extra_values: HashMap<String, String> = HashMap::new();
    let obj = slf
        .python_classes
        .get(&rule_status.id)
        .unwrap()
        .to_object(py);
    let obj_for_extra = obj.downcast::<types::PyType>(py).unwrap();
    let extra_names =
        make_errors::extra_from_class(obj_for_extra, MESSAGE_WITH_EXTRA_FROM_CLASS_PY).unwrap();
    //=========================================================================
    // let regex = ;
    for capture in check_convert::convert::string_to_default_regex(regex).captures_iter(text) {
        for name in &extra_names {
            flag = true;
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
    // Если указан `MustBeFoundhere`, но случайно указан `{}` в template, заполняем заглушкой
    /*
            class NoKeyFound:
            template = "Не найден ключ"
            rules = {r"key=\d+?": It.MustBeFoundHere}
    */
    // Если необходимо получить данные от ошибки должно быть `NotToBeFoundHere`, после
    // указываем название группы для получения результата
    if extra_values.is_empty() {
        for blank in extra_names {
            extra_values.insert(blank, format!( " \n| Do not use `{{ ... }}` along with `MustBeFoundHere`, specify what you want to find in `{}` | ",MESSAGE_WITH_EXTRA_FROM_CLASS_PY).to_string());
        }
    }
    make_errors::error_or_ok(&obj, extra_values, rule_status, flag)
}
