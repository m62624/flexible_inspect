use crate::rule::regex_types;

use super::*;

impl TemplateValidator {
    pub fn core_validate_single(&self, py: Python, text: String) -> PyObject {
        // Список ошибок
        let list_errors = types::PyList::new::<PyObject, _>(py, []);
        // Проверка по каждому правилу
        self.all_rules
            .iter()
            .map(|rule_container| {

                // rule.0.get_inner().
            })
            .for_each(drop);
        return list_errors.to_object(py);
    }

    pub fn work_on_one_rule_container(
        py: Python,
        text: String,
        rule_container: &(rule::Rule, usize),
    ) -> PyResult<()> {
        if let Some(rule_container) = rule_container.0.get_inner().as_ref() {
            match rule_container.1 {
                regex_types::RGX::Default => {
                    let matches = TemplateValidator::find_capture_default(&rule_container.0, &text);
                    Ok(())
                }
                regex_types::RGX::Fancy => {
                    let matches = TemplateValidator::find_capture_fancy(&rule_container.0, &text);
                    matches.iter().map(|x| {
                        // x.iter().map
                    });
                    Ok(())
                }
            }
        } else {
            Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                "* If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:

                x = Rule(\"x\")
                x.extend(Rule(\"Y\"))
                
                * Please use this syntax:
                
                x = Rule(\"x\").extend(Rule(\"y\"))
                * or 
                x = Rule(\"x\")
                x = x.extend(Rule(\"y\"))"
            )))
        }
    }
}

#[test]
fn show_body_capture() {
    dbg!(TemplateValidator::find_capture_default(
        r"(?P<say_my_name>\d+)(\w+)",
        r"123 456 aboba baboba"
    ));
}
