use super::*;

#[pymethods]
impl Rule {
    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            let (mut default_r_vec, mut fancy_r_vec) = (Vec::new(), Vec::new());
            list.iter().map(|packed_rule| {
                    if let Ok(rule) = packed_rule.extract::<Rule>() {
                        match rule.get_str_raw().unwrap() {
                            RegexRaw::DefaultR(_) => default_r_vec.push(rule),
                            RegexRaw::FancyR(_) => fancy_r_vec.push(rule),
                        }
                        Ok(())
                    } else {
                        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                            "Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",
                            packed_rule.get_type().name().unwrap(),
                            self.str_raw.as_ref().unwrap().as_ref()
                        )));
                    }
                }).collect::<PyResult<Vec<_>>>()?;
            if !default_r_vec.is_empty() || !fancy_r_vec.is_empty() {
                self.subrules = Some(Subrules::new(default_r_vec, fancy_r_vec));
            }
            return Ok(std::mem::take(self));
        }
        Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "`{}` -- Expected `List` --> List[Rule, Rule, Rule]",
            nested_rules.as_ref(py).get_type().name().unwrap()
        )))
    }
}
