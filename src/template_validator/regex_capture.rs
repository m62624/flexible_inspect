use super::*;
impl TemplateValidator {
    /// Поиск совпадений, используя `default regex`
    pub fn find_default_capture<'a>(pattern: &'a str, text: &'a str) -> Vec<regex::Captures<'a>> {
        check_convert::convert::str_to_default_regex(pattern)
            .captures_iter(text)
            .map(|captures| captures)
            .collect()
    }

    /// Поиск совпадений, используя `fancy regex`
    pub fn find_fancy_capture<'a>(
        pattern: &'a str,
        text: &'a str,
    ) -> Vec<fancy_regex::Captures<'a>> {
        check_convert::convert::str_to_fancy_regex(pattern)
            .captures_iter(text)
            .filter_map(|captures| captures.ok())
            .collect()
    }
}
