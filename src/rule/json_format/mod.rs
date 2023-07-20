use super::*;

impl Serialize for SerializableRegexSet {
    /// Сериализует `RegexSet` в `Vec<String>`
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let patterns = self.regex_set.patterns();
        patterns.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SerializableRegexSet {
    /// Десериализует `Vec<&str>` в `RegexSet`
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let patterns: Vec<&str> = Deserialize::deserialize(deserializer)?;
        let regex_set = regex::RegexSet::new(&patterns).map_err(DeError::custom)?;
        Ok(Self { regex_set })
    }
}
