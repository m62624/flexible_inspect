use super::rule::Rule;
use super::*;
mod unit_tests;
// use rule::
#[derive(Debug)]
pub enum MultiCapture<'a> {
    DefaultCaptures(Vec<regex::Captures<'a>>),
    FancyCaptures(Vec<fancy_regex::Captures<'a>>),
}

impl<'a> MultiCapture<'a> {
    pub fn is_some(&self) -> bool {
        match self {
            MultiCapture::DefaultCaptures(captures) => !captures.is_empty(),
            MultiCapture::FancyCaptures(captures) => !captures.is_empty(),
        }
    }
    pub fn find_captures(rule: &Rule, text: &'a str) -> PyResult<MultiCapture<'a>> {
        match rule.get_str_raw()? {
            rule::RegexRaw::DefaultR(pattern) => Ok(MultiCapture::DefaultCaptures(
                regex::Regex::new(&pattern)
                    .unwrap()
                    .captures_iter(text)
                    .map(|captures| captures)
                    .collect(),
            )),
            rule::RegexRaw::FancyR(pattern) => Ok(MultiCapture::FancyCaptures(
                fancy_regex::Regex::new(&pattern)
                    .unwrap()
                    .captures_iter(text)
                    .filter_map(|captures| captures.ok())
                    .collect(),
            )),
        }
    }
    pub fn to_str_vec(&self) -> Vec<&'a str> {
        match self {
            MultiCapture::DefaultCaptures(captures) => captures
                .iter()
                .flat_map(|capture| {
                    capture
                        .iter()
                        .filter_map(|capture| capture.map(|value| value.as_str()))
                        .collect::<Vec<_>>()
                })
                .collect(),
            MultiCapture::FancyCaptures(captures) => captures
                .iter()
                .flat_map(|capture| {
                    capture
                        .iter()
                        .filter_map(|capture| capture.map(|value| value.as_str()))
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}