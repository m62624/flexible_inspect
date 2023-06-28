mod step_by_step;
use super::rule::Rule;
use super::*;
mod getters;
#[derive(Debug)]
/// --> TemplateValidator
pub struct ExceptionContainer {
    py_class: PyObject,
    default_r: Vec<Rule>,
    fancy_r: Vec<Rule>,
}
