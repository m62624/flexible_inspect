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

    fn work_on_one_rule_container(
        py: Python,
        text: String,
        rule_container: &(rule::Rule, usize),
    ) -> PyResult<()> {
        let mut step: Option<Vec<String>> = Default::default();
        loop {
            step = Some(TemplateValidator::step_by_step_on_the_rule(
                step,
                Some(rule_container),
                &text,
            )?);
            if step.is_none()
                || (step.is_some() && rule_container.0.get_rules_for_the_rule().is_none())
            {
                break;
            }
        }
        Ok(())
    }

    fn step_by_step_on_the_rule(
        matches: Option<Vec<String>>,
        rule_container: Option<&(rule::Rule, usize)>,
        text: &str,
    ) -> PyResult<Vec<String>> {
        if let Some(rule_container) = rule_container {
            if let Some(rule_container) = rule_container.0.get_inner() {
                return Ok(match rule_container.1 {
                    regex_types::RGX::Default => TemplateValidator::default_captures_to_vec_string(
                        TemplateValidator::find_capture_default(&rule_container.0, &text),
                    ),
                    regex_types::RGX::Fancy => TemplateValidator::fancy_captures_to_vec_string(
                        TemplateValidator::find_capture_fancy(&rule_container.0, &text),
                    ),
                });
            } else {
                return Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                    "* If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:
    
                    x = Rule(\"x\")
                    x.extend(Rule(\"Y\"))
                    
                    * Please use this syntax:
                    
                    x = Rule(\"x\").extend(Rule(\"y\"))
                    * or 
                    x = Rule(\"x\")
                    x = x.extend(Rule(\"y\"))"
                )));
            }
        }
        Ok(vec![])
    }
}

#[test]
fn show_body_capture() {
    // dbg!(
    let x = TemplateValidator::find_capture_default(
        r"(?P<say_my_name>\d+)\W+",
        r"123 456 aboba baboba",
    );
    dbg!(&x);
    let strings_vec: Vec<String> = x
        .iter()
        .flat_map(|captures| {
            captures
                .iter()
                .filter_map(|capture| capture.map(|value| value.as_str().to_string()))
        })
        .collect();
    dbg!(strings_vec);
    // ));
}

#[test]
fn test_step_by_step_on_the_rule() {
    let rls = rule::Rule::new(
        r"<h>.+</h>".to_string(),
        match_requirement::MatchRequirement::MustBeFound,
    );
}
