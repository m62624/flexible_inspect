//===========================
// t_x - TRUE (match)
// e_x - Err() (SHOULD_PANIC)
//===========================

// Unit тесты
#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(test)]
    mod convert_tests {
        use super::*;

        mod fn_bytes_to_string_utf8 {
            use super::*;

            #[test]
            fn bytes_to_string_utf8_t_0() {
                assert_eq!(
                    convert::bytes_to_string_utf8("!!! 😊 😎 & 🚀".as_bytes()).unwrap(),
                    "!!! 😊 😎 & 🚀"
                );
            }

            #[test]
            #[should_panic]
            fn bytes_to_string_utf8_f_0() {
                pyo3::prepare_freethreaded_python();
                convert::bytes_to_string_utf8(b"\xF0\x90\x80").unwrap();
            }
        }

        mod fn_string_to_default_regex {
            use super::*;

            #[test]
            fn string_to_default_regex_t_0() {
                assert_eq!(
                    convert::string_to_default_regex(String::from("[0-9]+?")).to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            #[test]
            #[should_panic]
            fn string_to_default_regex_f_0() {
                convert::string_to_default_regex(String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }

            #[test]
            #[should_panic(
                expected = "error: look-around, including look-ahead and look-behind, is not supported"
            )]
            fn string_to_default_regex_f_1() {
                convert::string_to_default_regex(String::from(r"(\b\w+\b)(?=.+?\1)"));
            }
        }

        mod fn_string_to_fancy_regex {
            use super::*;
            #[test]
            fn string_to_fancy_regex_t_0() {
                assert_eq!(
                    convert::string_to_default_regex(String::from("[0-9]+?")).to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            #[test]
            fn string_to_fancy_regex_t_1() {
                convert::string_to_fancy_regex(String::from(r"(\b\w+\b)(?=.+?\1)"));
            }

            #[test]
            #[should_panic]
            fn string_to_fancy_regex_f_0() {
                convert::string_to_fancy_regex(String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }
        }
    }
}
