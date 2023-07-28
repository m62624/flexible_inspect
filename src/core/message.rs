use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn filling_message(
    message_template: String,
    error_data: Option<HashMap<String, String>>,
) -> String {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\{(.+?)\}").unwrap();
    }
    let result = RE.replace_all(&message_template, |caps: &regex::Captures| {
        let key = caps.get(1).unwrap().as_str();
        if let Some(value) = error_data.as_ref().and_then(|data| data.get(key)) {
            value
        } else {
            "___"
        }
    });
    result.to_string()
}
