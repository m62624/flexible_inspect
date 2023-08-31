use indexmap::IndexSet;

use crate::rules::TypeStorageFormat;

use super::*;

/// Проверяем, что работает `Captures`, `Default Regex`
#[test]
fn fn_find_captures_t_0() {
    let text = "1 2 3 4 567";
    let rule = Rule::new(r"(?P<aboba>\d+)", MatchRequirement::MustBeFound);
    let captures_1 = Rule::find_captures(&rule, &text);
    let captures_2 = IndexSet::from(["1", "2", "3", "4", "567"]);
    if let TypeStorageFormat::Single(value) = captures_1.text_for_capture {
        assert!(value.0.is_subset(&captures_2));
        assert_eq!(
            captures_1.hashmap_for_error.get(DEFAULT_CAPTURE).unwrap(),
            "1"
        );
    } else {
        panic!("error");
    }
}

/// Проверяем, что работает `Captures`, `Fancy Regex`
#[test]
fn find_captures_t_1() {
    let text = "1 2 3 4 567 123";
    let rule = Rule::new(r"(?P<aboba>\d+(?=\d))", MatchRequirement::MustBeFound);
    let captures_1 = Rule::find_captures(&rule, &text);
    let captures_2 = IndexSet::from(["12", "56"]);
    if let TypeStorageFormat::Single(value) = captures_1.text_for_capture {
        assert!(value.0.is_subset(&captures_2));
        assert_eq!(
            captures_1.hashmap_for_error.get(DEFAULT_CAPTURE).unwrap(),
            "56"
        );
    } else {
        panic!("error");
    }
}
