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

    // Тестируем модуль конвертаций
    #[cfg(test)]
    mod convert {
        use super::*;

        // Тесты для функций fn_bytes_to_string_utf8
        mod fn_bytes_to_string_utf8 {
            use super::*;
            // Проверка UTF8
            #[test]
            fn bytes_to_string_utf8_t_0() {
                assert_eq!(
                    check_convert::convert::bytes_to_string_utf8("!!! 😊 😎 & 🚀".as_bytes())
                        .unwrap(),
                    "!!! 😊 😎 & 🚀"
                );
            }

            // Проверка UTF8
            #[test]
            #[should_panic]
            fn bytes_to_string_utf8_f_0() {
                pyo3::prepare_freethreaded_python();
                check_convert::convert::bytes_to_string_utf8(b"\xF0\x90\x80").unwrap();
            }
        }

        // Тесты для функций fn_string_to_default_regex
        mod fn_string_to_default_regex {
            use super::*;

            // Проверка корректной конветраций в Default Regex
            #[test]
            fn string_to_default_regex_t_0() {
                assert_eq!(
                    check_convert::convert::string_to_default_regex(&String::from("[0-9]+?"))
                        .to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            // Проверка корректной конветраций в Default Regex
            #[test]
            #[should_panic]
            fn string_to_default_regex_f_0() {
                check_convert::convert::string_to_default_regex(&String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }

            // Проверка корректной конветраций в Default Regex
            #[test]
            #[should_panic(
                expected = "error: look-around, including look-ahead and look-behind, is not supported"
            )]
            fn string_to_default_regex_f_1() {
                check_convert::convert::string_to_default_regex(&String::from(
                    r"(\b\w+\b)(?=.+?\1)",
                ));
            }
        }

        // Тесты для функций fn_string_to_fancy_regex
        mod fn_string_to_fancy_regex {
            use super::*;

            // Проверка корректной конветраций в Fancy Regex
            #[test]
            fn string_to_fancy_regex_t_0() {
                assert_eq!(
                    check_convert::convert::string_to_default_regex(&String::from("[0-9]+?"))
                        .to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            // Проверка корректной конветраций в Fancy Regex
            #[test]
            fn string_to_fancy_regex_t_1() {
                check_convert::convert::string_to_fancy_regex(&String::from(r"(\b\w+\b)(?=.+?\1)"));
            }

            // Проверка корректной конветраций в Fancy Regex
            #[test]
            #[should_panic]
            fn string_to_fancy_regex_f_0() {
                check_convert::convert::string_to_fancy_regex(&String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }
        }
    }

    // Тестируем модуль проверки
    mod check_tests {
        use super::*;

        // Тесты для функций fn_is_default_regex_fisrt_step
        mod fn_is_default_regex_fisrt_step {
            use super::*;

            // Проверка на возможность конвертаций в Default Regex
            #[test]
            fn is_default_regex_fisrt_step_t_0() {
                assert_eq!(
                    check_convert::check::is_default_regex_fisrt_step("[0-9]+"),
                    true
                );
            }

            // Проверка на возможность конвертаций в Default Regex
            #[test]
            fn is_default_regex_fisrt_step_t_1() {
                assert_eq!(
                    check_convert::check::is_default_regex_fisrt_step(
                        r"\QThis is not a valid regex!@#$%^&*()_+\E"
                    ),
                    false
                );
            }

            // Проверка на возможность конвертаций в Default Regex
            #[test]
            fn is_default_regex_fisrt_step_t_2() {
                assert_eq!(
                    check_convert::check::is_default_regex_fisrt_step(r"(\b\w+\b)(?=.+?\1)"),
                    false
                );
            }
        }

        // Тесты для функций fn_fancy_regex_second_step
        mod fn_fancy_regex_second_step {
            use super::*;

            // Проверка на возможность конвертаций в Fancy Regex
            #[test]
            fn is_fancy_regex_second_step_t_0() {
                assert_eq!(
                    check_convert::check::is_fancy_regex_second_step("[0-9]+"),
                    true
                );
            }

            // Проверка на возможность конвертаций в Fancy Regex
            #[test]
            fn is_fancy_regex_second_step_t_1() {
                assert_eq!(
                    check_convert::check::is_fancy_regex_second_step(
                        r"\QThis is not a valid regex!@#$%^&*()_+\E"
                    ),
                    false
                );
            }

            // Проверка на возможность конвертаций в Fancy Regex
            #[test]
            fn is_fancy_regex_second_step_t_2() {
                assert_eq!(
                    check_convert::check::is_fancy_regex_second_step(r"(\b\w+\b)(?=.+?\1)"),
                    true
                );
            }
        }
    }

    // Тестируем модуль инициализаций данных
    mod init_tests {
        use super::*;

        // Функция для использования повторно
        fn fn_core_get_any_regex_from_class(
            rules: &[(&str, It)],
            all_simple_rules: &mut HashMap<String, RuleStatus>,
            all_hard_rules: &mut HashMap<String, RuleStatus>,
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

        // Тесты для функций fn_get_any_regex_from_class
        mod fn_get_any_regex_from_class {

            // Структура для теста на иного объект
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

            // Проверка на правильное распределение
            // регулярных выражений между HashMap в fn_get_any_regex_from_class
            use super::*;
            #[test]
            fn fn_get_any_regex_from_class_t_0() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                let mut all_simple_rules = HashMap::new();
                let mut all_hard_rules = HashMap::new();
                let mut selected_simple_rules = Vec::new();
                fn_core_get_any_regex_from_class(
                    &[
                        ("rule1", It::MustBeFoundHere),
                        ("rule2", It::NotToBeFoundHere),
                        (r"(\b\w+\b)(?=.+?\1)", It::NotToBeFoundHere),
                    ],
                    &mut all_simple_rules,
                    &mut all_hard_rules,
                    &mut selected_simple_rules,
                    2,
                    1,
                    2,
                )
            }

            // Проверка на правильное распределение
            // регулярных выражений между HashMap в fn_get_any_regex_from_class
            #[test]
            fn fn_get_any_regex_from_class_t_1() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                let mut all_simple_rules = HashMap::new();
                let mut all_hard_rules = HashMap::new();
                let mut selected_simple_rules = Vec::new();
                fn_core_get_any_regex_from_class(
                    &[
                        ("rule1", It::MustBeFoundHere),
                        ("rule2", It::MustBeFoundHere),
                        ("rule3", It::MustBeFoundHere),
                        ("rule4", It::MustBeFoundHere),
                        ("rule2", It::NotToBeFoundHere),
                        (r"(\b\w+\b)(?=.+?\1)", It::NotToBeFoundHere),
                    ],
                    &mut all_simple_rules,
                    &mut all_hard_rules,
                    &mut selected_simple_rules,
                    4,
                    1,
                    4,
                )
            }

            // Проверка на правильное распределение
            // регулярных выражений между HashMap, при
            // других типах переменных в fn_get_any_regex_from_class
            #[test]
            #[should_panic(expected = r"PyErr { type: <class 'TypeError'>")]
            fn fn_get_any_regex_from_class_e_0() {
                pyo3::prepare_freethreaded_python();
                let mut all_simple_rules = HashMap::new();
                let mut all_hard_rules = HashMap::new();
                let mut selected_simple_rules = Vec::new();
                fn_core_get_any_regex_from_class(
                    &[
                        ("rule1", It::MustBeFoundHere),
                        ("rule2", It::MustBeFoundHere),
                        ("rule3", It::MustBeFoundHere),
                        ("rule4", It::MustBeFoundHere),
                        (
                            r"\QThis is not a valid regex!@#$%^&*()_+\E",
                            It::NotToBeFoundHere,
                        ),
                        (r"(\b\w+\b)(?=.+?\1)", It::NotToBeFoundHere),
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

            // Проверка на правильное распределение
            // регулярных выражений между HashMap, при
            // других типах переменных в fn_get_any_regex_from_class
            #[test]
            #[should_panic(expected = r#"'None' must be a 'String"#)]
            fn fn_get_any_regex_from_class_e_1() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let rules = [(py.None(), It::MustBeFoundHere)];
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

            // Проверка на правильное распределение
            // регулярных выражений между HashMap, при
            // других типах переменных в fn_get_any_regex_from_class
            #[test]
            #[should_panic(expected = r#"'True' must be a 'Enum'"#)]
            fn fn_core_get_any_regex_from_class_e_2() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let rules = &[("rule1", FakeObj::__new__())];
                    let mut all_simple_rules: HashMap<String, RuleStatus> = HashMap::new();
                    let mut all_hard_rules: HashMap<String, RuleStatus> = HashMap::new();
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

            // Проверка на правильное распределение
            // регулярных выражений между HashMap, при
            // других типах переменных в fn_get_any_regex_from_class
            #[test]
            #[should_panic(expected = r#" must be a 'dict"#)]
            fn fn_core_get_any_regex_from_class_e_3() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let mut all_simple_rules: HashMap<String, RuleStatus> = HashMap::new();
                    let mut all_hard_rules: HashMap<String, RuleStatus> = HashMap::new();
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

            // Проверка на отсутствие атрибута (элемент конструктора)
            #[test]
            #[should_panic(expected = r"AttributeError")]
            fn fn_core_get_any_regex_from_class_e_4() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let mut all_simple_rules: HashMap<String, RuleStatus> = HashMap::new();
                    let mut all_hard_rules: HashMap<String, RuleStatus> = HashMap::new();
                    let mut selected_simple_rules: Vec<String> = Vec::new();
                    let fake_class = types::PyType::new::<It>(py);
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

        // Тесты для функций fn_data_unpackaging
        mod fn_data_unpackaging {
            use super::*;

            // Проверка элемента конструктора (расфасовка данных)
            #[test]
            fn data_unpackaging_t_0() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| -> PyResult<()> {
                    let mut all_simple_rules: HashMap<String, RuleStatus> = HashMap::new();
                    let mut all_hard_rules: HashMap<String, RuleStatus> = HashMap::new();
                    let mut selected_simple_rules: Vec<String> = Vec::new();
                    let mut python_classes: HashMap<usize, PyObject> = HashMap::new();

                    let rules = &[
                        ("rule1", It::MustBeFoundHere),
                        ("rule2", It::NotToBeFoundHere),
                        (r"(\b\w+\b)(?=.+?\1)", It::NotToBeFoundHere),
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

            // Проверка элемента конструктора (расфасовка данных)
            #[test]
            fn data_unpackaging_t_1() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| -> PyResult<()> {
                    let rules = &[
                        ("rule1", It::MustBeFoundHere),
                        ("rule2", It::NotToBeFoundHere),
                        (r"(\b\w+\b)(?=.+?\1)", It::NotToBeFoundHere),
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

            // Проверка входных данных с Python для конструктора
            #[test]
            #[should_panic(expected = r"'None' must be a 'List[ Class, Class... ]")]
            fn data_unpackaging_e_0() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let empty_obj = py.None();
                    TemplateValidator::__new__(empty_obj)
                })
                .unwrap();
            }

            // Проверка входных данных с Python для конструктора
            #[test]
            #[should_panic(expected = r"must be a 'Class'")]
            fn data_unpackaging_e_1() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let mut all_simple_rules: HashMap<String, RuleStatus> = HashMap::new();
                    let mut all_hard_rules: HashMap<String, RuleStatus> = HashMap::new();
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

    // Тестируем модуль по созданию ошибок
    mod make_errors_tests {

        // Вспомогательная  функция по создание  Py объектов
        fn create_obj(rules: Option<&[(&str, It)]>, msg: &str) -> PyResult<PyObject> {
            Python::with_gil(|py| -> PyResult<PyObject> {
                let dict = types::PyDict::new(py);
                if let Some(rules) = rules {
                    for (key, value) in rules.iter() {
                        dict.set_item(key, Py::new(py, value.to_owned()).unwrap())?;
                    }
                }
                let class = types::PyType::new::<TemplateValidator>(py);
                class.setattr(RULES_FROM_CLASS_PY, dict)?;
                class.setattr(
                    MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                    types::PyString::new(py, format!("{}", msg).as_str()),
                )?;
                Ok(class.to_object(py))
            })
        }
        use super::*;

        // Тесты для функций fn_create_error
        mod fn_create_error {
            use super::*;

            // Проверяем статус, что ошибка создана
            #[test]
            fn create_error_t_0() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let obj = py.eval("object()", None, None).unwrap();
                    let result = make_errors::create_error(&obj.to_object(py), None).is_err();
                    assert_eq!(result, true);
                })
            }

            // Проверяем статус, что ошибка создана и можно сделать проброс
            #[test]
            #[should_panic]
            fn create_error_e_0() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| {
                    let empty_obj = py.None();
                    make_errors::create_error(&empty_obj, None).unwrap();
                });
            }
        }

        // Тесты для функций fn_extra_from_class
        mod fn_extra_from_class {
            use super::*;

            // Проврка получения **extra значений
            #[test]
            fn extra_from_class_t_1() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| -> PyResult<()> {
                    let rules = &[
                        (r"(?P<x>rule1)", It::MustBeFoundHere),
                        ("rule2", It::NotToBeFoundHere),
                        (r"(\b\w+\b)(?=.+?\1)", It::NotToBeFoundHere),
                    ];
                    let class_py = create_obj(Some(rules), "message {x}")?;
                    let extra = make_errors::extra_from_class(
                        &class_py.to_object(py).downcast(py).unwrap(),
                        MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                    )?;
                    assert_eq!(extra[0], "x");
                    Ok(())
                })
            }

            // Проврка получения **extra значений
            #[test]
            fn extra_from_class_e_0() -> PyResult<()> {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|py| -> PyResult<()> {
                    let rules = &[
                        (r"(?P<x>rule1)", It::MustBeFoundHere),
                        ("rule2", It::NotToBeFoundHere),
                        (r"(\b\w+\b)(?=.+?\1)", It::NotToBeFoundHere),
                    ];
                    let class_py = create_obj(Some(rules), "message")?;
                    make_errors::extra_from_class(
                        &class_py.to_object(py).downcast(py).unwrap(),
                        MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                    )?;
                    Ok(())
                })
            }
        }

        // Тесты функций fn_error_or_ok
        mod fn_error_or_ok {
            use super::*;

            #[test]
            // Проверка результата при условий
            // (It::MustBeFoundHere, true) => Ok(())
            fn error_or_ok_t_0() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|_| {
                    let class_py = create_obj(None, "x").unwrap();
                    make_errors::error_or_ok(
                        &class_py,
                        HashMap::new(),
                        &RuleStatus {
                            id: 1,
                            status: It::MustBeFoundHere,
                        },
                        true,
                    )
                    .unwrap();
                });
            }

            // Проверка результата при условий
            // (It::NotToBeFoundHere, false) => Ok(())
            #[test]
            fn error_or_ok_t_1() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|_| {
                    let class_py = create_obj(None, "x").unwrap();
                    make_errors::error_or_ok(
                        &class_py,
                        HashMap::new(),
                        &RuleStatus {
                            id: 1,
                            status: It::NotToBeFoundHere,
                        },
                        false,
                    )
                    .unwrap();
                });
            }

            // Проверка результата при условий
            // (It::MustBeFoundHere, false) =>  error()
            #[test]
            #[should_panic]
            fn error_or_ok_e_0() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|_| {
                    let class_py = create_obj(None, "x").unwrap();
                    make_errors::error_or_ok(
                        &class_py,
                        HashMap::new(),
                        &RuleStatus {
                            id: 1,
                            status: It::MustBeFoundHere,
                        },
                        false,
                    )
                    .unwrap();
                });
            }

            // Проверка результата при условий
            // (It::NotToBeFoundHere, true) =>  error()
            #[test]
            #[should_panic]
            fn error_or_ok_e_1() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|_| {
                    let class_py = create_obj(None, "x").unwrap();
                    make_errors::error_or_ok(
                        &class_py,
                        HashMap::new(),
                        &RuleStatus {
                            id: 1,
                            status: It::NotToBeFoundHere,
                        },
                        true,
                    )
                    .unwrap();
                });
            }

            // Проверка результата при условий
            // (It::NotToBeFoundHere, true) =>  error() // some extra
            #[test]
            #[should_panic]
            fn error_or_ok_e_2() {
                pyo3::prepare_freethreaded_python();
                Python::with_gil(|_| {
                    let class_py = create_obj(None, "x").unwrap();
                    let mut extra: HashMap<String, String> = HashMap::new();
                    extra.insert(String::from("x"), String::from("101"));
                    make_errors::error_or_ok(
                        &class_py,
                        extra,
                        &RuleStatus {
                            id: 1,
                            status: It::NotToBeFoundHere,
                        },
                        true,
                    )
                    .unwrap();
                });
            }
        }
    }
    // Тестирование модуля валидаций
    mod validate_tests {
        use super::*;

        // Праверка на совпадение, успешный возращение результата Ok(),
        #[test]
        fn switch_loop_regex_t_0() {
            let d_regex = r"[0-9]+?".to_string();
            // let f_rege = r"(\b\w+\b)(?=.+?\1)";
            let extra_names: Vec<String> = Vec::new();
            let mut extra_values: HashMap<String, String> = HashMap::new();
            let mut flag_status: bool = true;
            let def_regex: bool = true;
            let text: &str = "123";
            validate::switch_loop_regex(
                &d_regex,
                &extra_names,
                &mut extra_values,
                &mut flag_status,
                def_regex,
                text,
            )
            .unwrap();
        }
        // (DEFAULT REGEX) Праверка на совпадение, успешный возращение результата Ok(),
        // успешное совпадение **extra
        #[test]
        fn switch_loop_regex_t_1() {
            let d_regex = r"(?P<x>[0-9]+?)".to_string();
            // let f_rege = r"(\b\w+\b)(?=.+?\1)";
            let mut extra_names: Vec<String> = Vec::new();
            let mut extra_values: HashMap<String, String> = HashMap::new();
            let text: &str = "123";
            extra_names.push("x".to_string());
            let mut flag_status: bool = true;
            let def_regex: bool = true;
            validate::switch_loop_regex(
                &d_regex,
                &extra_names,
                &mut extra_values,
                &mut flag_status,
                def_regex,
                text,
            )
            .unwrap();
        }
        // (DEFAULT REGEX) Праверка на совпадение, успешный возращение результата Ok(),
        // отсутсвует **extra, заполняем пустышкой "___"
        #[test]
        fn switch_loop_regex_t_2() {
            let d_regex = r"(?P<x>[0-9]+?)".to_string();
            // let f_rege = r"(\b\w+\b)(?=.+?\1)";
            let mut extra_names: Vec<String> = Vec::new();
            let mut extra_values: HashMap<String, String> = HashMap::new();
            let text: &str = "123";
            extra_names.push("y".to_string());
            let mut flag_status: bool = true;
            let def_regex: bool = true;
            validate::switch_loop_regex(
                &d_regex,
                &extra_names,
                &mut extra_values,
                &mut flag_status,
                def_regex,
                text,
            )
            .unwrap();
        }
        // (FANCY REGEX) Праверка на совпадение, успешный возращение результата Ok(),
        // успешное совпадение **extra
        #[test]
        fn switch_loop_regex_t_3() {
            // let d_regex = r"(?P<x>[0-9]+?)".to_string();
            let f_regex = r"(?P<x>[0-9]+?)".to_string();
            let mut extra_names: Vec<String> = Vec::new();
            let mut extra_values: HashMap<String, String> = HashMap::new();
            let text: &str = "123";
            extra_names.push("x".to_string());
            let mut flag_status: bool = true;
            let def_regex: bool = false;
            validate::switch_loop_regex(
                &f_regex,
                &extra_names,
                &mut extra_values,
                &mut flag_status,
                def_regex,
                text,
            )
            .unwrap();
        }
        // (FANCY REGEX) Праверка на совпадение, успешный возращение результата Ok(),
        // отсутсвует **extra, заполняем пустышкой "___"
        #[test]
        fn switch_loop_regex_t_4() {
            let f_regex = r"(\b\w+\b)(?=.+?\1)".to_string();
            let mut extra_names: Vec<String> = Vec::new();
            let mut extra_values: HashMap<String, String> = HashMap::new();
            let text: &str = "text text";
            extra_names.push("y".to_string());
            let mut flag_status: bool = true;
            let def_regex: bool = false;
            validate::switch_loop_regex(
                &f_regex,
                &extra_names,
                &mut extra_values,
                &mut flag_status,
                def_regex,
                text,
            )
            .unwrap();
        }
    }
}
