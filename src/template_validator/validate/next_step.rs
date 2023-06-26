use super::*;
use crate::template_validator::captures::MultiCapture;
use py_exception::*;
pub fn result_on_the_match(
    py: Python,
    rule: &rule::Rule,
    obj: &PyObject,
    captures: MultiCapture,
    next_step: &mut bool,
) -> PyResult<()> {
    match rule.get_requirement()? {
        rule::MatchRequirement::MustBeFound => {
            match (captures.is_some(), rule.get_op_subrules().is_some()) {
                (true, true) => {
                    *next_step = true;
                    // *error_flag = false;
                }
                (true, false) => {
                    *next_step = false;
                    // *error_flag = false;
                }
                (false, true) => {
                    *next_step = false;
                    make_error::init_error(
                        obj,
                        extra_collection::extra_from__captures(
                            extra_collection::extra_from_class(
                                py,
                                obj,
                                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                            )?,
                            captures,
                        ),
                    )?;
                    // *error_flag = true;
                }
                (false, false) => {
                    *next_step = false;
                    // *error_flag = true;
                    make_error::init_error(
                        obj,
                        extra_collection::extra_from__captures(
                            extra_collection::extra_from_class(
                                py,
                                obj,
                                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                            )?,
                            captures,
                        ),
                    )?;
                }
            }
        }
        rule::MatchRequirement::MustNotBefound => {
            match (captures.is_some(), rule.get_op_subrules().is_some()) {
                (true, true) => {
                    *next_step = true;
                    //  *error_flag = false;
                }
                (true, false) => {
                    *next_step = false;
                    // *error_flag = true;
                    make_error::init_error(
                        obj,
                        extra_collection::extra_from__captures(
                            extra_collection::extra_from_class(
                                py,
                                obj,
                                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                            )?,
                            captures,
                        ),
                    )?;
                }
                (false, true) => {
                    *next_step = false;
                    // *error_flag = false;
                }
                (false, false) => {
                    *next_step = false;
                    // *error_flag = false;
                }
            }
        }
    }
    Ok(())
}
/*
pub fn runner(
      py: Python,
      rule: &rule::Rule,
      all_rules: &Vec<(rule::Rule, usize)>,
      classes: &HashMap<usize, PyObject>,
      text: &str,
  ) -> PyResult<()> {
      let mut stack: VecDeque<&rule::Rule> = VecDeque::new();
      let mut error_flag = false;
      let mut next_step = false;
      stack.push_back(rule);
      while let Some(rule) = stack.pop_back() {
          match rule.get_requirement()? {
              rule::MatchRequirement::MustBeFound => {
                  let captures = TemplateValidator::find_default_capture(rule.get_str()?, text);
                  step_by_step::result_on_the_match(
                      rule,
                      !captures.is_empty(),
                      &mut error_flag,
                      &mut next_step,
                  )?;

                  if next_step {
                      if let Some(rgxst) =
                          rule::Rule::make_sub_regex_set(rule.get_op_subrules(), text)
                      {
                          check_convert::convert::str_from_default_capture(&captures)
                              .iter()
                              .map(|text| {
                                  rgxst
                                      .iter()
                                      .map(|id_pattern| {
                                          stack.push_back(&all_rules[*id_pattern].0);
                                          Ok(())
                                      })
                                      .collect::<PyResult<Vec<_>>>()
                              })
                              .collect::<PyResult<Vec<_>>>()?;
                      }
                  }
              }
              rule::MatchRequirement::MustNotBefound => {}
          }
      }

      Ok(())
  } */
