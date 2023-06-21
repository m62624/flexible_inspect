use super::*;
impl TemplateValidator {
    /// Поиск совпадений, используя `default regex`
    pub fn find_capture_default<'a>(pattern: &'a str, text: &'a str) -> Vec<regex::Captures<'a>> {
        check_convert::convert::string_to_default_regex(pattern)
            .captures_iter(text)
            .collect::<Vec<_>>()
    }
    /// Поиск совпадений, используя `fancy regex`
    pub fn find_capture_fancy<'a>(
        pattern: &'a str,
        text: &'a str,
    ) -> Vec<fancy_regex::Captures<'a>> {
        check_convert::convert::string_to_fancy_regex(pattern)
            .captures_iter(text)
            .filter_map(|result| result.ok())
            .collect::<Vec<_>>()
    }
}
