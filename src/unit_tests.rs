// Если использовать стандартный формат именование тестов MethodName_StateUnderTest_ExpectedBehavior,
// Название этих тестов могут быть очень длинными, поэтому я решил использовать более короткий вариант. (Не смотря на то что длинное название тестов показывает, что и при каких условиях идет тест
// в реальности мы сверяем `assert`, промежуточные и финальные данные, поэтому название тестов не дает полный картины проверки, как их результаты.)

// Пример :
//================(Синтаксис)================
// - fn_<имя_функции>_<t|e>_<номер_теста>
//===========================================

// 1) указываем само имя в функции в начале имени, дальше выбираем ожидаемый результат
// 2) t_x, где `t` - это что может быть положительно ПРИ СРАВНЕНИЙ, а `x` - номер теста

// 2.1 ) Пример :
//===========================
// t_x - TRUE (match)
// e_x - Err() / (SHOULD_PANIC)
//===========================

// 2.2)`t` : к примеру наша функция возращает `true` и мы ждем `true`.
// тоже самое подходит и для `false` и `false`, и даже если условие `false` не `true`.
// Все три варианта возращают `true` потому что
// мы используем СРАВНЕНИЕ, если ваше сравнение удачно, значит это `t`
// 2.3) `x` : это номер теста, вы можете указать несколько тестов которые ждут `t` | `e`,
// поэтому мы можем, просто повышать номер с тестом, главное правило,
// счетчик должен быть уникальным для каждой функций (именно то, что мы проверяем)
// и для каждого типа `t` / `e`, так мы избавлямся от дубликатов
// 3) e_x, где `e` - это что может быть положительно ПРИ ПАНИКЕ, а `x` - номер теста
// 3.1) `e` - то что возращает панику или проброс ошибки, так можно маркировать тесты
// которые `should_panic` либо те, где мы проверяем определенный тип ошибки `error == error`
// 3.2) `x` - пункт 2.3

// Допольнительные условия (не обязательно)

// 1) Желательно каждый unit тест который проверяет одну функцию вкладывать в отдельный модуль
// (само собой всё в один модуль, всё остальное `submodule`),
// это связано с тем, что на одну функцию может быть несколько тестов, и если мы будем писать
// для каждой функций свои модуль, то мы сможем легко найти тесты для определенной функции
// с повышением количество тестов. Именование модулей должно быть таким же как и у функций с префиксом `fn_`.

// 1.1) Пример :
//================(Синтаксис)================
// #[cfg(test)]
// mod tests {
//     #[cfg(test)]
//     mod fn_<имя_функции> {
//         use super::*;
//         // множество тестов для одной функций для разной обработки поведений и результатов
//     }
//     ... Повторяем для каждой функции
// }
//===========================================

// 2) если тесты пишутся для каждой ОС отдельно (к примеру для Windows и Linux),
// используйте разделение имен тестов в родительском модуле, а не в дочернем
// 3) комментируйте каким образом сравниваем, после промежуточные (если есть), финальные и просто, что сравнивается в assert, ЕСЛИ это не очевидно из самого кода

// Unit тесты
#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(test)]
    mod convert {
        use super::*;

        mod fn_bytes_to_string_utf8 {
            use super::*;
            #[test]
            fn bytes_to_string_utf8_t_0() {
                assert_eq!(
                    check_convert::convert::bytes_to_string_utf8("!!! 😊 😎 & 🚀".as_bytes())
                        .unwrap(),
                    "!!! 😊 😎 & 🚀"
                );
            }

            #[test]
            #[should_panic]
            fn bytes_to_string_utf8_f_0() {
                pyo3::prepare_freethreaded_python();
                check_convert::convert::bytes_to_string_utf8(b"\xF0\x90\x80").unwrap();
            }
        }

        mod fn_string_to_default_regex {
            use super::*;

            #[test]
            fn string_to_default_regex_t_0() {
                assert_eq!(
                    check_convert::convert::string_to_default_regex(String::from("[0-9]+?"))
                        .to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            #[test]
            #[should_panic]
            fn string_to_default_regex_f_0() {
                check_convert::convert::string_to_default_regex(String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }

            #[test]
            #[should_panic(
                expected = "error: look-around, including look-ahead and look-behind, is not supported"
            )]
            fn string_to_default_regex_f_1() {
                check_convert::convert::string_to_default_regex(String::from(
                    r"(\b\w+\b)(?=.+?\1)",
                ));
            }
        }

        mod fn_string_to_fancy_regex {
            use super::*;
            #[test]
            fn string_to_fancy_regex_t_0() {
                assert_eq!(
                    check_convert::convert::string_to_default_regex(String::from("[0-9]+?"))
                        .to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            #[test]
            fn string_to_fancy_regex_t_1() {
                check_convert::convert::string_to_fancy_regex(String::from(r"(\b\w+\b)(?=.+?\1)"));
            }

            #[test]
            #[should_panic]
            fn string_to_fancy_regex_f_0() {
                check_convert::convert::string_to_fancy_regex(String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }
        }
    }
    mod check_tests {
        use super::*;

        mod fn_is_default_regex_fisrt_step {
            use super::*;

            #[test]
            fn is_default_regex_fisrt_step_t_0() {
                assert_eq!(
                    check_convert::check::is_default_regex_fisrt_step("[0-9]+"),
                    true
                );
            }

            #[test]
            fn is_default_regex_fisrt_step_t_1() {
                assert_eq!(
                    check_convert::check::is_default_regex_fisrt_step(
                        r"\QThis is not a valid regex!@#$%^&*()_+\E"
                    ),
                    false
                );
            }

            #[test]
            fn is_default_regex_fisrt_step_t_2() {
                assert_eq!(
                    check_convert::check::is_default_regex_fisrt_step(r"(\b\w+\b)(?=.+?\1)"),
                    false
                );
            }
        }

        mod fn_fancy_regex_second_step {
            use super::*;

            #[test]
            fn is_fancy_regex_second_step_t_0() {
                assert_eq!(
                    check_convert::check::is_fancy_regex_second_step("[0-9]+"),
                    true
                );
            }

            #[test]
            fn is_fancy_regex_second_step_t_1() {
                assert_eq!(
                    check_convert::check::is_fancy_regex_second_step(
                        r"\QThis is not a valid regex!@#$%^&*()_+\E"
                    ),
                    false
                );
            }

            #[test]
            fn is_fancy_regex_second_step_t_2() {
                assert_eq!(
                    check_convert::check::is_fancy_regex_second_step(r"(\b\w+\b)(?=.+?\1)"),
                    true
                );
            }
        }
    }
    mod init_tests {
        use super::*;
        fn fn_core_get_any_regex_from_class(
            rules: &[(&str, IfFound)],
            all_simple_rules: &mut HashMap<RuleStatus, usize>,
            all_hard_rules: &mut HashMap<RuleStatus, usize>,
            selected_simple_rules: &mut Vec<String>,
            count_all_simple_rules: usize,
            count_all_hard_rules: usize,
            count_selected_simple_rules: usize,
        ) -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let dict = types::PyDict::new(py);
                for (key, value) in rules.iter() {
                    dict.set_item(key, Py::new(py, value.to_owned()).unwrap())?;
                }
                let class = types::PyType::new::<TemplateValidator>(py);
                class.setattr(RULES_FROM_CLASS_PY, dict)?;
                init::get_any_regex_from_class(
                    &class,
                    1,
                    all_simple_rules,
                    all_hard_rules,
                    selected_simple_rules,
                )?;
                assert_eq!(all_simple_rules.len(), count_all_simple_rules);
                assert_eq!(all_hard_rules.len(), count_all_hard_rules);
                assert_eq!(selected_simple_rules.len(), count_selected_simple_rules);
                Ok(())
            })
        }

        mod fn_get_any_regex_from_class {
            #[pyclass]
            struct FakeObj {
                status: bool,
            }
            #[pymethods]
            impl FakeObj {
                #[new]
                fn __new__() -> Self {
                    FakeObj { status: true }
                }
            }
            impl ToPyObject for FakeObj {
                fn to_object(&self, py: Python<'_>) -> PyObject {
                    self.status.to_object(py)
                }
            }
            use super::*;
            #[test]
            fn fn_get_any_regex_from_class_t_0() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                let mut all_simple_rules = HashMap::new();
                let mut all_hard_rules = HashMap::new();
                let mut selected_simple_rules = Vec::new();
                fn_core_get_any_regex_from_class(
                    &[
                        ("rule1", IfFound::AllRight),
                        ("rule2", IfFound::RaiseError),
                        (r"(\b\w+\b)(?=.+?\1)", IfFound::RaiseError),
                    ],
                    &mut all_simple_rules,
                    &mut all_hard_rules,
                    &mut selected_simple_rules,
                    2,
                    1,
                    2,
                )
            }
            #[test]
            fn fn_get_any_regex_from_class_t_1() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                let mut all_simple_rules = HashMap::new();
                let mut all_hard_rules = HashMap::new();
                let mut selected_simple_rules = Vec::new();
                fn_core_get_any_regex_from_class(
                    &[
                        ("rule1", IfFound::AllRight),
                        ("rule2", IfFound::AllRight),
                        ("rule3", IfFound::AllRight),
                        ("rule4", IfFound::AllRight),
                        ("rule2", IfFound::RaiseError),
                        (r"(\b\w+\b)(?=.+?\1)", IfFound::RaiseError),
                    ],
                    &mut all_simple_rules,
                    &mut all_hard_rules,
                    &mut selected_simple_rules,
                    4,
                    1,
                    4,
                )
            }
            #[test]
            #[should_panic(expected = r"PyErr { type: <class 'TypeError'>")]
            fn fn_get_any_regex_from_class_e_0() {
                pyo3::prepare_freethreaded_python();
                let mut all_simple_rules = HashMap::new();
                let mut all_hard_rules = HashMap::new();
                let mut selected_simple_rules = Vec::new();
                fn_core_get_any_regex_from_class(
                    &[
                        ("rule1", IfFound::AllRight),
                        ("rule2", IfFound::AllRight),
                        ("rule3", IfFound::AllRight),
                        ("rule4", IfFound::AllRight),
                        (
                            r"\QThis is not a valid regex!@#$%^&*()_+\E",
                            IfFound::RaiseError,
                        ),
                        (r"(\b\w+\b)(?=.+?\1)", IfFound::RaiseError),
                    ],
                    &mut all_simple_rules,
                    &mut all_hard_rules,
                    &mut selected_simple_rules,
                    4,
                    1,
                    4,
                )
                .unwrap()
            }

            #[test]
            #[should_panic(expected = r#"'None' must be a 'String"#)]
            fn fn_get_any_regex_from_class_e_1() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let rules = [(py.None(), IfFound::AllRight)];
                    let dict = types::PyDict::new(py);
                    for (key, value) in rules.iter() {
                        dict.set_item(key, Py::new(py, value.to_owned()).unwrap())
                            .unwrap();
                    }
                    let class = types::PyType::new::<TemplateValidator>(py);
                    class.setattr(RULES_FROM_CLASS_PY, dict).unwrap();
                    TemplateValidator::__new__(
                        types::PyList::new(py, [class.to_object(py).to_object(py)].iter())
                            .to_object(py),
                    )
                })
                .unwrap();
            }
            #[test]
            #[should_panic(expected = r#"'True' must be a 'Enum'"#)]
            fn fn_core_get_any_regex_from_class_e_2() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let rules = &[("rule1", FakeObj::__new__())];
                    let mut all_simple_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut all_hard_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut selected_simple_rules: Vec<String> = Vec::new();
                    let dict = types::PyDict::new(py);
                    for (key, value) in rules.iter() {
                        dict.set_item(key, value).unwrap();
                    }
                    let class = types::PyType::new::<TemplateValidator>(py);
                    class.setattr(RULES_FROM_CLASS_PY, dict).unwrap();
                    init::get_any_regex_from_class(
                        &class,
                        1,
                        &mut all_simple_rules,
                        &mut all_hard_rules,
                        &mut selected_simple_rules,
                    )
                    .unwrap();
                });
            }
            #[test]
            #[should_panic(expected = r#" must be a 'dict"#)]
            fn fn_core_get_any_regex_from_class_e_3() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let mut all_simple_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut all_hard_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut selected_simple_rules: Vec<String> = Vec::new();
                    let no_dict = types::PyBool::new(py, true);
                    let class = types::PyType::new::<TemplateValidator>(py);
                    class.setattr(RULES_FROM_CLASS_PY, no_dict).unwrap();
                    init::get_any_regex_from_class(
                        &class,
                        1,
                        &mut all_simple_rules,
                        &mut all_hard_rules,
                        &mut selected_simple_rules,
                    )
                    .unwrap();
                });
            }

            #[test]
            #[should_panic(expected = r"AttributeError")]
            fn fn_core_get_any_regex_from_class_e_4() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let mut all_simple_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut all_hard_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut selected_simple_rules: Vec<String> = Vec::new();
                    let fake_class = types::PyType::new::<IfFound>(py);
                    init::get_any_regex_from_class(
                        &fake_class,
                        0,
                        &mut all_simple_rules,
                        &mut all_hard_rules,
                        &mut selected_simple_rules,
                    )
                    .unwrap()
                });
            }
        }
        mod fn_data_unpackaging {
            use super::*;

            #[test]
            fn data_unpackaging_t_0() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| -> PyResult<()> {
                    let mut all_simple_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut all_hard_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut selected_simple_rules: Vec<String> = Vec::new();
                    let mut python_classes: HashMap<usize, PyObject> = HashMap::new();

                    let rules = &[
                        ("rule1", IfFound::AllRight),
                        ("rule2", IfFound::RaiseError),
                        (r"(\b\w+\b)(?=.+?\1)", IfFound::RaiseError),
                    ];
                    let dict = types::PyDict::new(py);
                    for (key, value) in rules.iter() {
                        dict.set_item(key, Py::new(py, value.to_owned()).unwrap())?;
                    }
                    let class = types::PyType::new::<TemplateValidator>(py);
                    class.setattr(RULES_FROM_CLASS_PY, dict)?;
                    let obj_main = types::PyList::new(py, [class].iter());
                    init::data_unpackaging(
                        py,
                        obj_main.to_object(py),
                        &mut python_classes,
                        &mut all_simple_rules,
                        &mut all_hard_rules,
                        &mut selected_simple_rules,
                    )
                })
            }
            #[test]
            fn data_unpackaging_t_1() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| -> PyResult<()> {
                    let rules = &[
                        ("rule1", IfFound::AllRight),
                        ("rule2", IfFound::RaiseError),
                        (r"(\b\w+\b)(?=.+?\1)", IfFound::RaiseError),
                    ];
                    let dict = types::PyDict::new(py);
                    for (key, value) in rules.iter() {
                        dict.set_item(key, Py::new(py, value.to_owned()).unwrap())?;
                    }
                    let class = types::PyType::new::<TemplateValidator>(py);
                    class.setattr(RULES_FROM_CLASS_PY, dict)?;
                    let obj_main = types::PyList::new(py, [class].iter());
                    TemplateValidator::__new__(obj_main.to_object(py))?;
                    Ok(())
                })
            }

            #[test]
            #[should_panic(expected = r#"'None' must be a 'List[ Class, Class... ]'")"#)]
            fn data_unpackaging_e_0() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let empty_obj = py.None();
                    TemplateValidator::__new__(empty_obj)
                })
                .unwrap();
            }
            #[test]
            #[should_panic(expected = r"must be a 'Class'")]
            fn data_unpackaging_e_1() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let mut all_simple_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut all_hard_rules: HashMap<RuleStatus, usize> = HashMap::new();
                    let mut selected_simple_rules: Vec<String> = Vec::new();
                    let mut python_classes: HashMap<usize, PyObject> = HashMap::new();
                    let obj1 = types::PyBool::new(py, true);
                    let obj_main = types::PyList::new(py, [obj1].iter());

                    init::data_unpackaging(
                        py,
                        obj_main.to_object(py),
                        &mut python_classes,
                        &mut all_simple_rules,
                        &mut all_hard_rules,
                        &mut selected_simple_rules,
                    )
                    .unwrap()
                });
            }
        }
    }
}
