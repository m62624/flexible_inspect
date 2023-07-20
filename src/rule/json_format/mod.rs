use pyo3::exceptions::PyValueError;

use super::*;

impl Serialize for SerializableRegexSet {
    /// Сериализует `RegexSet` в `Vec<String>`
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Получаем вектор шаблонов
        let patterns = self.regex_set.patterns();
        // Сериализуем вектор шаблонов
        patterns.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SerializableRegexSet {
    /// Десериализует `Vec<&str>` в `RegexSet`
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // десериализуем вектор шаблонов
        let patterns: Vec<&str> =
            Deserialize::deserialize(deserializer).map_err(DeError::custom)?;
        // компилируем в `RegexSet`
        let regex_set = regex::RegexSet::new(&patterns).unwrap();
        Ok(Self { regex_set })
    }
}

#[pymethods]
impl Rule {
    /// Сериализует правило в JSON
    pub fn serialize(&self) -> PyResult<String> {
        let json = serde_json::to_string(self)
            .map_err(|str| PyValueError::new_err(format!("Error serialize json: {}", str)));
        Ok(json?)
    }

    /// Десериализует правило из JSON
    #[staticmethod]
    pub fn deserialize(json: String) -> PyResult<Self> {
        let rule = serde_json::from_str(&json)
            .map_err(|str| PyValueError::new_err(format!("Error deserialize json: {}", str)));
        Ok(rule?)
    }
}
