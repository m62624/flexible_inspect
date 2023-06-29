use super::*;

impl ExceptionContainer {
    pub fn sync_run(&self, py: Python, text: &str) -> PyResult<()> {
        if let Some(regexSet) = &self.default_roots_set {
            let skip_r = Rule::get_selected_rules(regexSet, text);
            skip_r
                .iter()
                .map(|id| {
                    rule::Rule::run(
                        py,
                        text,
                        &self.default_roots_vec.as_ref().unwrap()[*id],
                        &self.py_class,
                    )
                })
                .collect::<PyResult<Vec<_>>>()?;

            self.default_roots_vec
                .as_ref()
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(id, _)| !skip_r.contains(id))
                .map(|(_, rule)| rule::Rule::run(py, text, rule, &self.py_class))
                .collect::<PyResult<Vec<_>>>()?;
        }
        if let Some(rules) = &self.fancy_root_vec {
            rules
                .iter()
                .map(|rule| rule::Rule::run(py, text, rule, &self.py_class))
                .collect::<PyResult<Vec<_>>>()?;
        }
        Ok(())
    }
    pub fn async_run(&self) -> PyResult<()> {
        Ok(())
    }
}
