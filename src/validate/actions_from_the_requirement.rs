use super::captures::MultiCapture;

use super::custom_error::py_error::*;
use super::*;

/*
- MustBeFound - если найдено и нет subrules  -> Stop(), Ok()
- MustBeFound -> Если Найдено и есть subrules -> Continue(), Ok()
  |
  | --> MustBeFound - Если не найдено и есть subrules -> Stop(), Error()
 \|/
- MustBeFound - Если не найдено и нет subrules -> Stop(), Error()
========================================================================
- MustNotBeFound - Если Найдено и нет subrules -> Stop(), Error()
- MustNotBeFound - Если Найдено и есть subrules -> Continue(), Ok()
   |
   | ---> MustNotBeFound - Если не найдено и есть subrules -> Stop(), Ok()
  \|/
- MustNotBeFound - Не найдено и нет subrules -> Stop(), Ok()
 */

pub fn next_or_error(
    py: Python,
    class_template: &PyObject,
    rule: &rule::Rule,
    captures: MultiCapture,
    next_step: &mut bool,
) -> PyResult<Option<PyObject>> {
    match rule.get_requirement().unwrap() {
        rule::MatchRequirement::MustBeFound => {
            match (captures.is_some(), rule.get_op_subrules().is_some()) {
                (true, true) => {
                    *next_step = true;
                    Ok(None)
                }
                (true, false) => {
                    *next_step = false;
                    Ok(None)
                }
                (false, true) => Ok(Some(make_error(
                    class_template,
                    filling_extra(&get_extra_from_class(py, class_template)?, captures),
                )?)),
                (false, false) => Ok(Some(make_error(
                    class_template,
                    filling_extra(&get_extra_from_class(py, class_template)?, captures),
                )?)),
            }
        }
        rule::MatchRequirement::MustNotBefound => {
            match (captures.is_some(), rule.get_op_subrules().is_some()) {
                (true, true) => {
                    *next_step = true;
                    Ok(None)
                }
                (true, false) => Ok(Some(make_error(
                    class_template,
                    filling_extra(&get_extra_from_class(py, class_template)?, captures),
                )?)),
                (false, true) => {
                    *next_step = false;
                    Ok(None)
                }
                (false, false) => {
                    *next_step = false;
                    Ok(None)
                }
            }
        }
    }
}
