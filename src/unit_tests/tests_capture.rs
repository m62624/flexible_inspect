use super::captures::CaptureType;
use super::*;
use std::collections::HashSet;
mod fn_find_captures {
    use super::*;

    mod without_counter {
        use super::*;

        /// Проверка на то, что Capture работает корректно с Default Regex
        #[test]
        fn find_captures_t_0() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let text = "lock 1";
            let rule = rule::Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
            if let CaptureType::Single(v) =
                captures::CaptureData::find_captures(&rule, text).text_for_capture
            {
                assert_eq!(v, HashSet::from(["lock", "1"]));
            } else {
                panic!("Wrong type of CaptureType (expected Single))");
            }
            Ok(())
        }

        /// Проверка на то, что Capture работает корректно с Fancy Regex
        #[test]
        fn find_captures_t_1() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let text = "lock 1";
            let rule = rule::Rule::spawn(r"\w+ (?=1)", MatchRequirement::MustBeFound)?;
            if let CaptureType::Single(v) =
                captures::CaptureData::find_captures(&rule, text).text_for_capture
            {
                assert_eq!(v, HashSet::from(["lock "]));
            } else {
                panic!("Wrong type of CaptureType (expected Single))");
            }
            Ok(())
        }

        /// Проверка на то, что все совпадения уникальны Default Regex
        #[test]
        fn find_captures_t_2() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let rule = Rule::spawn(r"\w", MatchRequirement::MustBeFound)?;
            let text = "Hello, world!";
            if let CaptureType::Single(mut v) =
                captures::CaptureData::find_captures(&rule, text).text_for_capture
            {
                for s in v.clone() {
                    if v.insert(s) {
                        panic!("Duplicate strings found {}", s);
                    };
                }
            } else {
                panic!("Wrong type of CaptureType (expected Single))");
            }
            Ok(())
        }

        /// Проверка на то, что все совпадения уникальны Fancy Regex
        #[test]
        fn find_captures_t_3() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let rule = Rule::spawn(r"\w(?=\w)", MatchRequirement::MustBeFound)?;
            let text = "Hello, worlddd!";
            if let CaptureType::Single(mut v) =
                captures::CaptureData::find_captures(&rule, text).text_for_capture
            {
                for s in v.clone() {
                    if v.insert(s) {
                        panic!("Duplicate strings found {}", s);
                    };
                }
            } else {
                panic!("Wrong type of CaptureType (expected Single))");
            }
            Ok(())
        }

        /// Проверка на то, что совпадения повторяются Default Regex
        #[test]
        fn find_captures_t_4() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let rule = Rule::spawn(r"\w", MatchRequirement::MustBeFound)?.duplicate_matches();
            let text = "Hello, world!";
            if let CaptureType::Multiple(v) =
                captures::CaptureData::find_captures(&rule, text).text_for_capture
            {
                assert_ne!(
                    v.iter().collect::<HashSet<_>>().len(),
                    v.len(),
                    "No duplicates found"
                );
            } else {
                panic!("Wrong type of CaptureType (expected Multiple)");
            }
            Ok(())
        }

        /// Проверка на то, что совпадения повторяются Fancy Regex
        #[test]
        fn find_captures_t_5() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let rule = Rule::spawn(r"\w(?=\w)", MatchRequirement::MustBeFound)?.duplicate_matches();
            let text = "Hello, wrlddd!";
            if let CaptureType::Multiple(v) =
                captures::CaptureData::find_captures(&rule, text).text_for_capture
            {
                assert_ne!(
                    v.iter().collect::<HashSet<_>>().len(),
                    v.len(),
                    "No duplicates found"
                );
            } else {
                panic!("Wrong type of CaptureType (expected Multiple)");
            }
            Ok(())
        }

        /// Проверка заполнения group_name в Default Regex
        #[test]
        fn find_captures_t_6() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let text = "lock 1";
            let rule = rule::Rule::spawn(r"(?P<test_group>\w+)", MatchRequirement::MustBeFound)?;
            assert_eq!(
                captures::CaptureData::find_captures(&rule, text).hashmap_for_error,
                HashMap::from([
                    ("main_capture".into(), "lock".into()),
                    ("test_group".into(), "lock".into())
                ])
            );
            Ok(())
        }

        /// Проверка заполнения group_name в Fancy Regex
        #[test]
        fn find_captures_t_7() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let text = "lock 1";
            let rule =
                rule::Rule::spawn(r"(?P<test_group>\w+ (?=1))", MatchRequirement::MustBeFound)?;
            assert_eq!(
                captures::CaptureData::find_captures(&rule, text).hashmap_for_error,
                HashMap::from([
                    ("main_capture".into(), "lock ".into()),
                    ("test_group".into(), "lock ".into())
                ])
            );
            Ok(())
        }
    }

    mod with_counter {
        use super::*;

        /// Проверка вычисления счетчика в Default Regex
        #[test]
        fn find_captures_t_0() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let text = "lock lock lock";
            let rule =
                rule::Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(5);
            assert_eq!(
                captures::CaptureData::find_captures(&rule, text).counter_value,
                3
            );
            Ok(())
        }

        /// Проверка вычисления счетчика в Fancy Regex
        #[test]
        fn find_captures_t_1() -> PyResult<()> {
            pyo3::prepare_freethreaded_python();
            let text = "lock lock lock";
            let rule = rule::Rule::spawn(r"(\w+ (?=lock))", MatchRequirement::MustBeFound)?
                .counter_is_equal(5);
            assert_eq!(
                captures::CaptureData::find_captures(&rule, text).counter_value,
                2
            );
            Ok(())
        }
    }
}

mod fn_is_some {
    use super::*;

    /// Проверка is_some если есть совпадение без дубликатов
    #[test]
    fn is_some_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let text = "lock 1";
        let rule = rule::Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
        assert!(captures::CaptureData::find_captures(&rule, text).is_some());
        Ok(())
    }

    /// Проверка is_some если есть совпадение с дубликатами
    #[test]
    fn is_some_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let text = "lock lock";
        let rule = rule::Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.duplicate_matches();
        assert!(captures::CaptureData::find_captures(&rule, text).is_some());
        Ok(())
    }
}
