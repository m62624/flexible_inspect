use super::*;

mod fn_find_captures {
    use super::*;

    #[test]
    fn find_captures_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let text = "lock 1";
        let rule = rule::Rule::spawn(r"(?P<ABOBA>\w+ (?=1))", MatchRequirement::MustBeFound)?;
        let captures = captures::CaptureData::find_captures(&rule, text);
        dbg!(captures);
        Ok(())
    }

    #[test]
    fn find_captures_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::spawn(r"\w", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello, world!";
        let captures = captures::CaptureData::find_captures(&rule, text);
        let mut unique = captures.text_for_capture;
        for s in unique.clone() {
            if unique.insert(&s) {
                panic!("Duplicate strings found {}", s);
            };
        }
        Ok(())
    }

    #[test]
    fn find_captures_t_2() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let text = "lock 1";
        let rule = rule::Rule::spawn(r"(?P<ABOBA>\w+)", MatchRequirement::MustBeFound)?;
        let captures = captures::CaptureData::find_captures(&rule, text);
        dbg!(captures);
        Ok(())
    }
}
