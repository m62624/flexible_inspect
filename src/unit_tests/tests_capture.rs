use super::*;
use std::collections::HashSet;
mod fn_find_captures {
    use super::*;

    /// Проверка на то, что Capture работает корректно с Fancy Regex
    #[test]
    fn find_captures_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let text = "lock 1";
        let rule = rule::Rule::spawn(r"(?P<group_1>\w+ (?=1))", MatchRequirement::MustBeFound)?;
        assert_eq!(
            captures::CaptureData::find_captures(&rule, text).text_for_capture,
            HashSet::from(["lock "])
        );
        Ok(())
    }

    /// Проверка на то, что Capture работает корректно с Default Regex
    #[test]
    fn find_captures_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let text = "lock 1";
        let rule = rule::Rule::spawn(r"(?P<ABOBA>\w+)", MatchRequirement::MustBeFound)?;
        let captures = captures::CaptureData::find_captures(&rule, text);
        dbg!(captures);
        Ok(())
    }

    /// Проверка на то, что все совпадения уникальны
    #[test]
    fn find_captures_t_2() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::spawn(r"\w", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello, world!";
        let mut captures = captures::CaptureData::find_captures(&rule, text);
        for s in captures.text_for_capture.clone() {
            if captures.text_for_capture.insert(s) {
                dbg!(&captures.text_for_capture);
                panic!("Duplicate strings found {}", s);
            };
        }
        Ok(())
    }
}
