use crate::core::captures::CaptureData;

use super::*;

#[test]
fn find_captures_t_0() {
    let byte_rule =
        Rule::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound).for_bytes();
    let hay = r"foo\x00qu\xFFux\x00baz\x00";
    // let captures = CaptureData::find_captures(&byte_rule, hay);
    // dbg!(captures);
}
