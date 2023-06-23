use super::*;
impl TemplateValidator {
    /// Поиск совпадений, используя `default regex`
    pub fn find_default_capture<'a>(pattern: &'a str, text: &'a str) -> Vec<&'a str> {
        check_convert::convert::string_to_default_regex(pattern)
            .captures_iter(text)
            .flat_map(|captures| {
                captures
                    .iter()
                    .filter_map(|capture| capture.map(|value| value.as_str()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Поиск совпадений, используя `fancy regex`
    pub fn find_fancy_capture<'a>(pattern: &'a str, text: &'a str) -> Vec<&'a str> {
        check_convert::convert::string_to_fancy_regex(pattern)
            .captures_iter(text)
            .filter_map(|result| result.ok())
            .flat_map(|captures| {
                captures
                    .iter()
                    .filter_map(|capture| capture.map(|value| value.as_str()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
