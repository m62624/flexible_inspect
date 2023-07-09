use super::cartridge::CartridgeWrapper;
use super::rule::next::NextStep;
use super::unit_tests::mock_obj::{self, CustomClassError};
use super::*;

mod fn_new {
    use super::*;

    /// Проверка создания объекта `CartridgeWrapper`
    #[test]
    fn new_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let class_py = mock_obj::make_obj(
                py,
                "GOTCHA : {data}",
                Some(vec![
                    Rule::spawn(r"\d", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"aboba", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"aboba(?=end)", MatchRequirement::MustBeFound)?.extend_t(
                        py,
                        vec![Rule::spawn("bob", MatchRequirement::MustNotBeFound)?],
                    )?,
                ]),
            );

            dbg!(CartridgeWrapper::new(py, class_py)?);
            Ok(())
        })
    }

    /// Проверка создания объекта `CartridgeWrapper`, указываем в `Rule` лишний объект != `Rule`
    #[test]
    #[should_panic]
    fn new_e_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let obj = types::PyType::new::<CustomClassError>(py);
            obj.setattr(
                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                types::PyString::new(py, ""),
            )
            .unwrap();
            obj.setattr(
                RULES_FROM_CLASS_PY,
                types::PyList::new(
                    py,
                    vec![
                        Rule::spawn(r"\d", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        Rule::spawn(r"\w", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        Rule::spawn(r"aboba", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        types::PyType::new::<CustomClassError>(py).into_py(py),
                    ],
                ),
            )
            .unwrap();
            CartridgeWrapper::new(py, obj.into_py(py)).unwrap();
        });
    }

    /// Проверка создания объекта `CartridgeWrapper`,
    /// указываем **один класс** вместо списка классов
    #[test]
    #[should_panic]
    fn new_e_1() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let obj = types::PyType::new::<CustomClassError>(py);
            obj.setattr(
                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                types::PyString::new(py, ""),
            )
            .unwrap();
            obj.setattr(
                RULES_FROM_CLASS_PY,
                types::PyType::new::<CustomClassError>(py),
            )
            .unwrap();
            CartridgeWrapper::new(py, obj.into_py(py)).unwrap();
        });
    }

    /// Проверка существование атрибута `Rules`
    /// (возможно нужно будет удалить это проверку, после наследования от `PystvalException`,
    /// должно быть на стороне пайтона проходить проверка)
    #[test]
    #[should_panic]
    fn new_e_2() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let obj = types::PyType::new::<CustomClassError>(py);
            obj.setattr(
                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                types::PyString::new(py, ""),
            )
            .unwrap();
            CartridgeWrapper::new(py, obj.into_py(py)).unwrap();
        });
    }

    /// Проверка конструктора, на элемент класса
    #[test]
    #[should_panic]
    fn new_e_3() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            CartridgeWrapper::new(py, types::PyBool::new(py, false).into_py(py)).unwrap();
        });
    }

    mod fn_run {

        use super::*;

        /// Проверка метода `run` на объекте `CartridgeWrapper`
        /// Все этапы проходят успешно
        #[test]
        fn run_t_0() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let class_py = mock_obj::make_obj(
                    py,
                    "GOTCHA : {data}",
                    Some(vec![
                        Rule::spawn(r"\w", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"aboba", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"aboba(?=end)", MatchRequirement::MustBeFound)?,
                    ]),
                );

                let cartridge = CartridgeWrapper::new(py, class_py)?;
                let text = "abobaend";
                let result = cartridge.sync_run(text);
                dbg!(&result);
                assert_eq!(result, NextStep::Finish);
                Ok(())
            })
        }

        #[test]
        fn run_t_1() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| -> PyResult<()> {
                let class_py = mock_obj::make_obj(
                    py,
                    "GOTCHA : {data}",
                    Some(vec![
                        Rule::spawn(r"\w", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"aba", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"aboba(?=end)", MatchRequirement::MustBeFound)?,
                    ]),
                );

                let cartridge = CartridgeWrapper::new(py, class_py)?;
                let text = "abobaend";
                let result = cartridge.sync_run(text);
                dbg!(&result);
                assert_eq!(result, NextStep::Error(None));
                Ok(())
            })
        }
    }
}