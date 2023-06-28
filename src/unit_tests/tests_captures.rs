use super::captures::MultiCapture;
use super::*;

#[cfg(test)]
mod fn_find_captures {
    use super::*;
    #[test]
    fn find_captures_t_0() {
        let rule = Rule::spawn(r"\w", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello, world!";
        let captures = MultiCapture::find_captures(&rule, text).unwrap();
        assert_eq!(
            captures.to_str_vec(),
            vec!["H", "e", "l", "l", "o", "w", "o", "r", "l", "d"]
        );
    }

    #[test]
    fn find_captures_t_1() {
        let rule = Rule::spawn(r"\w+ (?=\w+)", MatchRequirement::MustBeFound).unwrap();
        let text = "Hello world!";
        let captures = MultiCapture::find_captures(&rule, text).unwrap();
        assert_eq!(captures.to_str_vec(), vec!["Hello "]);
    }
}
