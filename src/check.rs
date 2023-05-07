use super::*;

impl TemplateValidator {
    /// Проверяем строку, является ли корректным регулярным выражением (`default regex`)\
    /// **Является первым шагом проверки во время инициализаций конструктора**
    fn is_default_regex_fisrt_step(line: &str) -> bool {
        regex::Regex::new(line).is_ok()
    }

    /// Проверяем строку, является ли корректным регулярным выражением (`fancy regex`)\
    /// **Является вторым шагом проверки во время инициализаций конструктора**
    fn is_fancy_regex_second_step(line: &str) -> bool {
        fancy_regex::Regex::new(&line).is_ok()
    }
}

/// Проверяем объект, что он является `PyList`
fn is_convertible_to_pylist(py_obj: &PyAny) -> bool {
    <types::PyList as pyo3::PyTryFrom>::try_from(py_obj).is_ok()
}

/// Проверяем объект, что он является `PyType`
fn is_convertible_to_pytype(py_obj: &PyAny) -> bool {
    <types::PyType as pyo3::PyTryFrom>::try_from(py_obj).is_ok()
}

// Эти unit тесты не находятся по пути `src/tests.rs`, потому что `Private` методы нельзя тестировать
// вне модуля, в котором они находятся
#[cfg(test)]
mod tests {
    use super::*;

    mod fn_is_default_regex_fisrt_step {
        use super::*;

        #[test]
        fn is_default_regex_fisrt_step_t_0() {
            assert_eq!(
                TemplateValidator::is_default_regex_fisrt_step("[0-9]+"),
                true
            );
        }

        #[test]
        fn is_default_regex_fisrt_step_t_1() {
            assert_eq!(
                TemplateValidator::is_default_regex_fisrt_step(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E"
                ),
                false
            );
        }

        #[test]
        fn is_default_regex_fisrt_step_t_2() {
            assert_eq!(
                TemplateValidator::is_default_regex_fisrt_step(r"(\b\w+\b)(?=.+?\1)"),
                false
            );
        }
    }

    mod fn_fancy_regex_second_step {
        use super::*;

        #[test]
        fn is_fancy_regex_second_step_t_0() {
            assert_eq!(
                TemplateValidator::is_fancy_regex_second_step("[0-9]+"),
                true
            );
        }

        #[test]
        fn is_fancy_regex_second_step_t_1() {
            assert_eq!(
                TemplateValidator::is_fancy_regex_second_step(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E"
                ),
                false
            );
        }

        #[test]
        fn is_fancy_regex_second_step_t_2() {
            assert_eq!(
                TemplateValidator::is_fancy_regex_second_step(r"(\b\w+\b)(?=.+?\1)"),
                true
            );
        }
    }
}
