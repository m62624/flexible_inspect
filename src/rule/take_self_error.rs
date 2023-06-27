use super::*;
use pyo3::exceptions;
pub fn absence_error() -> PyErr {
    PyErr::new::<exceptions::PyValueError, _>(format!(
        "* If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:

       x = Rule(\"X\")
       x.extend(Rule(\"Y\"))
       
       * Please use this syntax:
       
       x = Rule(\"X\").extend(Rule(\"Y\"))
       * or 
       x = Rule(\"X\")
       x = x.extend(Rule(\"Y\"))"
    ))
}
