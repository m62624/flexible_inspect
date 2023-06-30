use super::rule::Rule;
use super::*;

#[derive(Debug)]
pub enum MultiCapture<'a> {
    DefaultCaptures(Vec<regex::Captures<'a>>),
    FancyCaptures(Vec<fancy_regex::Captures<'a>>),
}

impl<'a> MultiCapture<'a> {
    pub fn find_captures(rule: &Rule, text: &'a str) -> PyResult<MultiCapture<'a>> {
        match &rule.get_content().unwrap().str_with_type {
            rule::RegexRaw::DefaultR(pattern) => Ok(MultiCapture::DefaultCaptures(
                regex::Regex::new(pattern)
                    .unwrap()
                    .captures_iter(text)
                    .collect(),
            )),
            rule::RegexRaw::FancyR(pattern) => Ok(MultiCapture::FancyCaptures(
                fancy_regex::Regex::new(pattern)
                    .unwrap()
                    .captures_iter(text)
                    .filter_map(|captures| captures.ok())
                    .collect(),
            )),
        }
    }
    pub fn is_some(&self) -> bool {
        match self {
            MultiCapture::DefaultCaptures(captures) => !captures.is_empty(),
            MultiCapture::FancyCaptures(captures) => !captures.is_empty(),
        }
    }
}

mod traits {
    use super::*;

    impl<'a> From<MultiCapture<'a>> for Vec<&'a str> {
        fn from(val: MultiCapture<'a>) -> Self {
            match val {
                MultiCapture::DefaultCaptures(captures) => {
                    let mut data = captures
                        .into_iter()
                        .flat_map(|capture| {
                            capture
                                .iter()
                                .filter_map(|capture| capture.map(|value| value.as_str()))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    data.dedup();
                    data
                }
                MultiCapture::FancyCaptures(captures) => {
                    let mut data = captures
                        .into_iter()
                        .flat_map(|capture| {
                            capture
                                .iter()
                                .filter_map(|capture| capture.map(|value| value.as_str()))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    data.dedup();
                    data
                }
            }
        }
    }
}
