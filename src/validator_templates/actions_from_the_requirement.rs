use super::captures::MultiCapture;
use super::custom_error::py_error::*;
use super::*;

/*
- MustBeFound - если найдено и нет subrules  -> Stop(), Ok()
- MustBeFound -> если найдено и есть subrules -> Continue(), Ok()
  |
  | --> MustBeFound - если не найдено и есть subrules -> Stop(), Error()
 \|/
- MustBeFound - если не найдено и нет subrules -> Stop(), Error()
========================================================================
- MustNotBeFound - если Найдено и нет subrules -> Stop(), Error()
- MustNotBeFound - если Найдено и есть subrules -> Continue(), Ok()
   |
   | ---> MustNotBeFound - если не найдено и есть subrules -> Stop(), Ok()
  \|/
- MustNotBeFound - если не найдено и нет subrules -> Stop(), Ok()
 */

pub fn next_or_error(
    py: Python,
    class_template: &PyObject,
    rule: &rule::Rule,
    captures: &MultiCapture,
) -> PyResult<bool> {
    match rule.get_content().unwrap().requirement {
        rule::MatchRequirement::MustBeFound => {
            match (
                captures.is_some(),
                rule.get_content().unwrap().subrules.is_some(),
            ) {
                (true, true) => Ok(true),
                (true, false) => Ok(false),
                (false, true) => Err(make_error(
                    class_template,
                    filling_extra(&get_extra_from_class(py, class_template)?, captures),
                )
                .unwrap_err()),
                (false, false) => Err(make_error(
                    class_template,
                    filling_extra(&get_extra_from_class(py, class_template)?, captures),
                )
                .unwrap_err()),
            }
        }
        rule::MatchRequirement::MustNotBefound => {
            match (
                captures.is_some(),
                rule.get_content().unwrap().subrules.is_some(),
            ) {
                (true, true) => Ok(true),
                (true, false) => Err(make_error(
                    class_template,
                    filling_extra(&get_extra_from_class(py, class_template)?, captures),
                )
                .unwrap_err()),
                (false, true) => Ok(false),
                (false, false) => Ok(false),
            }
        }
    }
}
