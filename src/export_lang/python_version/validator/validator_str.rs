use super::*;

#[pyclass(name = "TemplateValidator")]
pub struct PyTemplateValidator(TemplateValidator<Vec<Cartridge<Rule>>, Box<str>>);

// #[pymethods]
// impl PyTemplateValidator {
//     #[new]
//     fn new(py: Python, cartridges: Vec<PyCartridge>) -> PyResult<Self> {
//         let rust_cartridges = cartridges
//             .into_iter()
//             .map(|mut c| c.to_rust())
//             .collect::<Vec<_>>();
//         let x: TemplateValidator<Vec<Cartridge<Rule>>, &str> =
//             TemplateValidator::new(rust_cartridges);
//         Self { 0: x };
//         todo!()
//     }
// }
