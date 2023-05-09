use super::*;
impl TemplateValidator {
    // Работает только один цикл, в один момент времени
    pub fn core_validate(&self, text: String) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            // Первый цикл - простые запросы (не включает в себя : look-around, including look-ahead & look-behind)
            for match_idx in self.selected_simple_rules.matches(&text).iter() {
                // dbg!("Цикл - Простой запрос +");
                let rule = &self.selected_simple_rules.patterns()[match_idx];
                just_look_at_this(
                    py,
                    self,
                    rule,
                    true,
                    self.all_simple_rules.get(rule).unwrap(),
                    &text,
                )?;
            }
            // Второй цикл - простые запросы (проверка тех что не включены в `selected_simple_rules`)
            for (rule, rule_status) in self.all_simple_rules.iter() {
                // dbg!("Цикл -  Точный поиск +");
                just_look_at_this(py, self, rule, true, rule_status, &text)?;
            }
            //  Третий цикл - сложные запросы (всё что входит : look-around, including look-ahead & look-behind)
            for (rule, rule_status) in self.all_hard_rules.iter() {
                // dbg!("Цикл -  Сложный поиск +");
                just_look_at_this(py, self, rule, false, rule_status, &text)?;
            }
            Ok(())
        })
    }
}

pub fn just_look_at_this(
    py: Python,
    slf: &TemplateValidator,
    regex: &String,
    def_regex: bool,
    rule_status: &RuleStatus,
    text: &str,
) -> PyResult<()> {
    let obj = slf.python_classes[&rule_status.id].to_object(py);
    let obj_for_extra = obj.downcast::<types::PyType>(py).unwrap();
    let extra_names =
        make_errors::extra_from_class(obj_for_extra, MESSAGE_WITH_EXTRA_FROM_CLASS_PY)?;

    let mut extra_values = HashMap::new();
    let mut flag = false;
    switch_loop_regex(
        regex,
        &extra_names,
        &mut extra_values,
        &mut flag,
        def_regex,
        text,
    )?;
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
            extra_values.insert(blank, format!(
                " \n| Do not use `{{ ... }}` along with `MustBeFoundHere`, specify what you want to find in `{}` | ",
                MESSAGE_WITH_EXTRA_FROM_CLASS_PY
            ));
        }
    }
    make_errors::error_or_ok(&obj, extra_values, rule_status, flag)
}

pub fn switch_loop_regex(
    regex: &String,
    extra_names: &Vec<String>,
    extra_values: &mut HashMap<String, String>,
    flag_status: &mut bool,
    def_regex: bool,
    text: &str,
) -> PyResult<()> {
    if def_regex {
        for capture in check_convert::convert::string_to_default_regex(regex).captures_iter(text) {
            *flag_status = true;
            for name in extra_names {
                match capture.name(&name) {
                    Some(value) => {
                        extra_values.insert(name.to_string(), value.as_str().to_string());
                    }
                    None => {
                        extra_values.insert(name.to_string(), "___".to_string());
                    }
                }
            }
        }
    } else {
        for capture in check_convert::convert::string_to_fancy_regex(regex).captures_iter(text) {
            *flag_status = true;
            for name in extra_names {
                match capture.as_ref().unwrap().name(&name) {
                    Some(value) => {
                        extra_values.insert(name.to_string(), value.as_str().to_string());
                    }
                    None => {
                        extra_values.insert(name.to_string(), "___".to_string());
                    }
                }
            }
        }
    }
    Ok(())
}
