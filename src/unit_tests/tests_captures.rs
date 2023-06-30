use super::captures::MultiCapture;
use super::*;

mod fn_find_captures {
    use super::*;

    /// Проверка поиска всех захватов (`Regex Captures`)
    #[test]
    fn find_captures_t_0() {
        let rule = Rule::spawn(r"\w", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello, world!";
        let captures = MultiCapture::find_captures(&rule, text).unwrap();
        assert_eq!(
            Into::<Vec<&str>>::into(captures),
            vec!["H", "e", "l", "o", "w", "o", "r", "l", "d"]
        );
    }

    /// Проверка поиска всех захватов (`Fancy Captures`)
    #[test]
    fn find_captures_t_1() {
        let rule = Rule::spawn(r"\w+ (?=\w+)", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello world!";
        let captures = MultiCapture::find_captures(&rule, text).unwrap();
        assert_eq!(Into::<Vec<&str>>::into(captures), vec!["Hello "]);
    }
}

mod fn_is_some {
    use super::*;

    #[test]
    fn is_some_t_0() {
        let rule = Rule::spawn(r"\w", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello, world!";
        let captures = MultiCapture::find_captures(&rule, text).unwrap();
        assert_eq!(captures.is_some(), true);
    }

    #[test]
    fn is_some_t_1() {
        let rule = Rule::spawn(r"\w+ (?=\w+)", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello world!";
        let captures = MultiCapture::find_captures(&rule, text).unwrap();
        assert_eq!(captures.is_some(), true);
    }
}
