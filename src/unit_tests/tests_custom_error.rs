use super::captures::MultiCapture;
use super::mock_obj;
use super::*;

mod fn_make_error {
    use super::*;

    /// Проверяем создание объекта `PyErr` с помощью `Rule` и `MultiCapture`
    #[test]
    #[should_panic]
    fn make_error_t_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let text = "111 222 333";
            let obj_class = mock_obj::make_obj(py, "information found : {data}", None);
            let rule = rule::Rule::spawn(r"(?P<data>\d+)", MatchRequirement::MustBeFound).unwrap();
            let captures = MultiCapture::find_captures(&rule, text).unwrap();
            dbg!(&captures);
            let extra_names = custom_error::py_error::get_extra_from_class(py, &obj_class).unwrap();
            dbg!(&extra_names);
            let extra_with_values = custom_error::py_error::filling_extra(&extra_names, &captures);
            dbg!(&extra_with_values);
            dbg!(custom_error::py_error::make_error(&obj_class, extra_with_values).unwrap());
        });
    }

    /// Проверяем создание объекта `PyErr` с помощью `Rule` и `MultiCapture`
    #[test]
    #[should_panic]
    fn make_error_t_1() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let text = "111 222 333";
            let obj_class = mock_obj::make_obj(py, "information found : {data}", None);
            let rule =
                rule::Rule::spawn(r"(?<data>\d+ (?=\d+))", MatchRequirement::MustBeFound).unwrap();
            let captures = MultiCapture::find_captures(&rule, text).unwrap();
            dbg!(&captures);
            let extra_names = custom_error::py_error::get_extra_from_class(py, &obj_class).unwrap();
            dbg!(&extra_names);
            let extra_with_values = custom_error::py_error::filling_extra(&extra_names, &captures);
            dbg!(&extra_with_values);
            dbg!(custom_error::py_error::make_error(&obj_class, extra_with_values).unwrap());
        })
    }
}
