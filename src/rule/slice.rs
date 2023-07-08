use super::*;

/// Перечисление для делений правил, зависимо от того это корневое правило или подправило
pub enum RuleContext<'py> {
    Root(&'py types::PyType),
    Subelement(&'py Rule),
}

impl<'py> RuleContext<'py> {
    // Делим на простые и сложные правила
    pub fn slice_rules(
        // Контекст, для того, чтобы понять, что это за правило
        element: RuleContext,
        // Список правил
        all_rules: &types::PyList,
        // Вектор для простых правил
        simple_rules: &mut Vec<Rule>,
        // Вектор для сложных правил
        complex_rules: &mut Vec<Rule>,
    ) -> PyResult<()> {
        // Проходимся по списку правил
        all_rules
            .iter()
            .map(|packed_rule| {
                // Проверяем, что это правило
                if let Ok(rule) = packed_rule.extract::<Rule>() {
                    // Делим правила на простые и сложные, зависит от `RegexRaw`
                    match rule.content_unchecked().str_with_type {
                        RegexRaw::DefaultR(_) => simple_rules.push(rule),
                        RegexRaw::FancyR(_) => complex_rules.push(rule),
                    }
                    Ok(())
                } else {
                    // Если это не правило, то возвращаем ошибку
                    match &element {
                        RuleContext::Root(class_py) => {
                            let err_msg = format!(
                                "'{}' must be a 'Rule' from class `{}`",
                                packed_rule.get_type().name().unwrap(),
                                class_py.name().unwrap()
                            );
                            // ================= (LOG) =================
                            error!("{}",err_msg);
                            // =========================================
                            Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
                        }
                        // Если это не подправило, то возвращаем ошибку
                        RuleContext::Subelement(this_rule) => {
                            let err_msg = format!("Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",packed_rule, this_rule.content_unchecked().str_with_type.as_ref());
                            // ================= (LOG) =================
                            error!("{}",err_msg);
                            // =========================================
                            Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
                        },
                    }
                }
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}
