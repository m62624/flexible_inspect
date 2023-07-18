use super::slice::RuleContext;
use super::*;

mod counter;
mod duplicates;
mod matching_mode;

/// Реалзиуем методы для использование из `Python`
#[pymethods]
impl Rule {
    /// add subrules
    pub fn extend(&mut self, py: Python, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            // Создаем заранее два вектора, для простых и сложных правил
            let mut simple_collection: Vec<Rule> = Vec::new();
            let mut complex_collection: Vec<Rule> = Vec::new();
            // Заполняем эти векторы правилами
            RuleContext::slice_rules(
                RuleContext::Subelement(self),
                list,
                &mut simple_collection,
                &mut complex_collection,
            )?;
            // Если хотя бы один из векторов не пустой, то добавляем их в подправила
            if !simple_collection.is_empty() || !complex_collection.is_empty() {
                self.content_mut_unchecked().subrules = Some(Subrules::new(
                    SimpleRules::new(simple_collection),
                    complex_collection,
                ));

                // ================= (LOG) =================
                debug!(
                    "used the `extend` modifier for `Rule` ( `{}` )",
                    self.content_unchecked().str_with_type.as_ref()
                );
                // =========================================

                // Возвращаем правило, которое было взято из владения
                return Ok(std::mem::take(self));
            }
        }
        let err_msg = format!(
            "`{}` -- expected `List` --> List [Rule, Rule, Rule]",
            nested_rules
        );

        // ================= (LOG) =================
        error!("{}", err_msg);
        // =========================================

        Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
    }
}
