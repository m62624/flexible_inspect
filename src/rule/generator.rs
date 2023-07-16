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
        let gen_reg = RegExpBuilder::from(&texts[..]).build();
        debug!(
            "regular expression `{}` is generated based on the obtained data: \n {:#?}",
            gen_reg, texts
        );
        Rule::new(gen_reg, match_requirement)
    }
}
