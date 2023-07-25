// use crate::core::rules::{captures::CaptureType, traits::RuleBytesExtendBase};
// use std::collections::HashSet;

// use super::*;

// /// Проверяем, что работает `Captures`, Regex Bytes
// #[test]
// fn find_captures_t_0() {
//     let text = b"foo\x00qu\xFFux\x00baz\x00";
//     let rule = RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound);
//     let captures_1 = RuleBytes::find_captures(&rule, text);
//     let captures_2 = HashSet::from([
//         CaptureType::Bytes(&b"foo\0"[..]),
//         CaptureType::Bytes(&b"qu\xFFux\0"[..]),
//         CaptureType::Bytes(&b"baz\0"[..]),
//     ]);
//     assert!(captures_1.text_for_capture.is_subset(&captures_2));
// }
