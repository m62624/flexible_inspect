use lazy_static::lazy_static;
use std::collections::HashMap;

/// This function replaces the data in the error message with the data from the `error_data` parameter

// Get extra from the class, this is necessary if there are `extra` variables that are not in the
// `captures` (`Option<HashMap<String, String>>>`). This way we can
// fill `___` with an empty `___` so that there will be no `KeyError`.

/*
Let's say the messages have `{name}` and `{age}`. But `captures` has only `{name}`.
Then we will get a `KeyError` error because `extra_with_value` does not have the `age` key.
For this, we get `extra` from the class and fill `extra_with_value` with an empty `___`.
*/

pub fn filling_message(
    message_template: &str,
    error_data: Option<HashMap<String, String>>,
) -> String {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\{(.+?)\}").unwrap();
    }
    let result = RE.replace_all(message_template, |caps: &regex::Captures| {
        let key = caps.get(1).unwrap().as_str();
        if let Some(value) = error_data.as_ref().and_then(|data| data.get(key)) {
            value
        } else {
            "___"
        }
    });
    result.to_string()
}
