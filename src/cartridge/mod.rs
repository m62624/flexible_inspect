use super::rule::slice::RuleContext;
use super::rule::{SimpleRules, Subrules as RootRules};
use super::*;

mod runner;

#[derive(Debug)]

pub struct Cartridge {
    py_class: PyObject,
    root_rules: RootRules,
}
#[derive(Debug)]
pub struct CartridgeWrapper(Arc<Cartridge>);

impl CartridgeWrapper {
    pub fn new(py: Python, py_class: PyObject) -> PyResult<Self> {
        if let Ok(class_py) = py_class.downcast::<types::PyType>(py) {
            if let Ok(py_list) = class_py.getattr(RULES_FROM_CLASS_PY) {
                if let Ok(py_list) = py_list.downcast::<types::PyList>() {
                    let mut simple_rules = Vec::new();
                    let mut complex_rules = Vec::new();
                    RuleContext::slice_rules(
                        RuleContext::Root(class_py),
                        py_list,
                        &mut simple_rules,
                        &mut complex_rules,
                    )?;
                    return Ok(Self(Arc::new(Cartridge {
                        py_class,
                        root_rules: RootRules::new(SimpleRules::new(simple_rules), complex_rules),
                    })));
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "'{}' must be a 'List[ Rule, Rule... ]'",
                        py_list
                    )));
                }
            } else {
                return Err(PyErr::new::<exceptions::PyAttributeError, _>(format!(
                    "The class `{}` has no attribute : `{}`",
                    class_py, RULES_FROM_CLASS_PY
                )));
            }
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'Class'",
                py_class
            )));
        }
    }
}
