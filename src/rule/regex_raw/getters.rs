use super::*;
impl RegexRaw {
    pub fn get_str(&self) -> &str {
        match self {
            RegexRaw::DefaultR(value) => &value,
            RegexRaw::FancyR(value) => &value,
        }
    }
}
