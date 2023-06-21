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
    ) {
    }
}
