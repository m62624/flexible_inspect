use std::collections::HashSet;

use crate::core::{
    rules::{traits::RuleExtendBase, CaptureType},
    DEFAULT_CAPTURE,
};

use super::*;

/// Проверяем, что работает `Captures`, `Default Regex`
#[test]
fn find_captures_t_0() {
    let text = "1 2 3 4 567";
    let rule = Rule::new(r"(?P<aboba>\d+)", MatchRequirement::MustBeFound);
    let captures_1 = Rule::find_captures(&rule, text);
    let captures_2 = HashSet::from([
        CaptureType::Str("1"),
        CaptureType::Str("2"),
        CaptureType::Str("3"),
        CaptureType::Str("4"),
        CaptureType::Str("567"),
    ]);
    assert!(captures_1.text_for_capture.is_subset(&captures_2));
    assert_eq!(
        captures_1.hashmap_for_error.get(DEFAULT_CAPTURE).unwrap(),
        "1"
    );
}

/// Проверяем, что работает `Captures`, `Fancy Regex`
#[test]
fn find_captures_t_1() {
    let text = "1 2 3 4 567 123";
    let rule = Rule::new(r"(?P<aboba>\d+(?=\d))", MatchRequirement::MustBeFound);
    let captures_1 = Rule::find_captures(&rule, text);
    let captures_2 = HashSet::from([CaptureType::Str("12"), CaptureType::Str("56")]);
    assert!(captures_1.text_for_capture.is_subset(&captures_2));
    assert_eq!(
        captures_1.hashmap_for_error.get(DEFAULT_CAPTURE).unwrap(),
        "56"
    );
}
