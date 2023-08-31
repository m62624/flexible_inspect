use indexmap::IndexSet;

use crate::rules::TypeStorageFormat;

use super::*;

/// Проверяем, что работает `Captures`, Regex Bytes
#[test]
fn fn_find_captures_t_0() {
    let text = b"foo\x00qu\xFFux\x00baz\x00".as_slice();
    let rule = RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound);
    let captures_1 = RuleBytes::find_captures(&rule, &text);
    let captures_2 = IndexSet::from([&b"foo\0"[..], &b"qu\xFFux\0"[..], &b"baz\0"[..]]);
    if let TypeStorageFormat::Single(value) = captures_1.text_for_capture {
        assert!(value.0.is_subset(&captures_2));
    } else {
        panic!("error");
    }
}
