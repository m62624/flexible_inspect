use super::*;

pub enum RuleContext<'py> {
    Root(&'py types::PyType),
    Subelement(&'py Rule),
}

impl<'py> RuleContext<'py> {
    pub fn slice_rules(
        element: RuleContext,
        all_rules: &types::PyList,
        simple_rules: &mut Vec<Rule>,
        complex_rules: &mut Vec<Rule>,
    ) -> PyResult<()> {
        all_rules
            .iter()
            .map(|packed_rule| {
                if let Ok(rule) = packed_rule.extract::<Rule>() {
                    match rule.get_content().unwrap().str_with_type {
                        RegexRaw::DefaultR(_) => simple_rules.push(rule),
                        RegexRaw::FancyR(_) => complex_rules.push(rule),
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
                        RuleContext::Subelement(this_rule) => {
                            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!("Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",packed_rule, this_rule.get_content().unwrap().str_with_type.as_ref())));
                        },
                    }
                }
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}
