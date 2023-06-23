use crate::rule::regex_types;

use super::*;

impl TemplateValidator {
    pub fn core_validate_single(&self, py: Python, text: &str) -> PyObject {
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
}

pub fn action_on_the_matches(matches: &Vec<&str>, rule_contrainer: &rule::Rule) -> PyResult<bool> {
    match rule_contrainer.get_requirement()? {
        rule::MatchRequirement::MustBeFound => Ok(!matches.is_empty()),
        rule::MatchRequirement::MustNotBefound => {
            if rule_contrainer.is_exist_subrules() {
                Ok(!matches.is_empty())
            } else {
                Ok(matches.is_empty())
            }
        }
    }
}
