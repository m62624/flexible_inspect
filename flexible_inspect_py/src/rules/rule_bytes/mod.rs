mod modifiers;
use super::*;

#[pyclass(name = "RuleBytes")]
#[derive(Clone, Default)]
pub struct PyRuleBytes(pub(crate) Option<RuleBytes>);

#[pymethods]
impl PyRuleBytes {
    #[new]
    pub fn new(pattern: String, requirement: PyMatchRequeriment) -> Self {
        PyRuleBytes(Some(RuleBytes::new(pattern, requirement.into())))
    }
}

impl TryFrom<&mut PyRuleBytes> for PyRuleBytes {
    type Error = PyErr;

    fn try_from(value: &mut PyRuleBytes) -> Result<Self, Self::Error> {
        let value = std::mem::take(value);
        if value.0.is_some() {
            Ok(value)
        } else {
            Err(PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
        }
    }
}

impl TryFrom<PyRuleBytes> for RuleBytes {
    type Error = PyErr;

    fn try_from(value: PyRuleBytes) -> Result<Self, Self::Error> {
        value
            .0
            .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
    }
}
