use super::*;

pub enum RuleContext<'py> {
    Root(&'py types::PyType),
    Subelement(&'py Rule),
}

impl<'py> RuleContext<'py> {
    pub fn slice_rules(
        element: RuleContext,
        all_rules: &types::PyList,
        default_r_vec: &mut Vec<Rule>,
        fancy_r_vec: &mut Vec<Rule>,
    ) -> PyResult<()> {
        all_rules
            .iter()
            .map(|packed_rule| {
                if let Ok(rule) = packed_rule.extract::<Rule>() {
                    match rule.get_str_raw().unwrap() {
                        RegexRaw::DefaultR(_) => default_r_vec.push(rule),
                        RegexRaw::FancyR(_) => fancy_r_vec.push(rule),
                    }
                    Ok(())
                } else {
                    match &element {
                        RuleContext::Root(class_py) => {
                            Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                                "'{}' must be a 'Rule' from class `{}`",
                                packed_rule.get_type().name().unwrap(),
                                class_py.name().unwrap()
                            )))
                        }
                        RuleContext::Subelement(slf) => {
                            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!("Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",packed_rule, slf.str_raw.as_ref().unwrap().as_ref())));
                        },
                    }
                }
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}
