use super::*;
use grex::RegExpBuilder;

#[pymethods]
impl Rule {
    #[staticmethod]
    pub fn auto_generate(
        match_requirement: MatchRequirement,
        text: &types::PyList,
    ) -> PyResult<Rule> {
        let mut texts: Vec<&str> = Vec::new();
        for item in text.iter() {
            texts.push(
                item.downcast::<types::PyString>()
                    .map_err(|_| {
                        PyErr::new::<exceptions::PyTypeError, _>("text must be a list of strings")
                    })?
                    .to_str()?,
            );
        }
        Rule::new(RegExpBuilder::from(&texts[..]).build(), match_requirement)
    }
}
