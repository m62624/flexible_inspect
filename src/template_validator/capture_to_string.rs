use super::*;
impl TemplateValidator {
    fn default_captures_to_vec_string(captures: Vec<regex::Captures<'_>>) -> Vec<String> {
        captures
            .iter()
            .flat_map(|captures| {
                captures
                    .iter()
                    .filter_map(|capture| capture.map(|value| value.as_str().to_string()))
            })
            .collect::<Vec<String>>()
    }
    fn fancy_captures_to_vec_string(captures: Vec<fancy_regex::Captures<'_>>) -> Vec<String> {
        captures
            .iter()
            .flat_map(|captures| {
                captures
                    .iter()
                    .filter_map(|capture| capture.map(|value| value.as_str().to_string()))
            })
            .collect::<Vec<String>>()
    }
}
