use super::*;
pub enum MultiCapture<'c> {
    Default(Vec<regex::Captures<'c>>),
    Fancy(Vec<fancy_regex::Captures<'c>>),
}
impl Rule {
    pub fn find_captures<'a>(rule: &'a Rule, text: &'a str) -> PyResult<MultiCapture<'a>> {
        match rule.get_rule_raw()? {
            RuleType::DefaultRegex(value) => Ok(MultiCapture::Default(
                convert::str_to_default_regex(&value)
                    .captures_iter(text)
                    .map(|captures| captures)
                    .collect(),
            )),
            RuleType::FancyRegex(value) => Ok(MultiCapture::Fancy(
                convert::str_to_fancy_regex(&value)
                    .captures_iter(text)
                    .filter_map(|captures| captures.ok())
                    .collect(),
            )),
        }
    }
}

mod convert {
    use super::*;
    pub fn str_to_default_regex(line: &str) -> regex::Regex {
        regex::Regex::new(line).unwrap()
    }

    /// Конвертация `&str в `fancy regex`
    pub fn str_to_fancy_regex(line: &str) -> fancy_regex::Regex {
        fancy_regex::Regex::new(line).unwrap()
    }
    pub fn str_from_default_capture<'a>(captures: &MultiCapture<'a>) -> Vec<&'a str> {
        match captures {
            MultiCapture::Default(captures) => captures
                .iter()
                .flat_map(|capture| {
                    capture
                        .iter()
                        .filter_map(|capture| capture.map(|value| value.as_str()))
                        .collect::<Vec<_>>()
                })
                .collect(),
            MultiCapture::Fancy(captures) => captures
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

mod check {
    pub fn is_default_regex_fisrt_step(line: &str) -> bool {
        regex::Regex::new(line).is_ok()
    }
    /// Проверяем строку, является ли корректным регулярным выражением (`fancy regex`)\
    /// ** Является вторым шагом проверки во время инициализаций конструктора **
    pub fn is_fancy_regex_second_step(line: &str) -> bool {
        fancy_regex::Regex::new(line).is_ok()
    }
}
