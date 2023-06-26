use super::*;
pub fn result_on_the_match(
    rule: &rule::Rule,
    captures: bool,
    error_flag: &mut bool,
    next_step: &mut bool,
) -> PyResult<()> {
    match rule.get_requirement()? {
        rule::MatchRequirement::MustBeFound => match (captures, rule.get_op_subrules().is_some()) {
            (true, true) => {
                *next_step = true;
                *error_flag = false;
            }
            (true, false) => {
                *next_step = false;
                *error_flag = false;
            }
            (false, true) => {
                *next_step = false;
                *error_flag = true;
            }
            (false, false) => {
                *next_step = false;
                *error_flag = true;
            }
        },
        rule::MatchRequirement::MustNotBefound => {
            match (captures, rule.get_op_subrules().is_some()) {
                (true, true) => {
                    *next_step = true;
                    *error_flag = false;
                }
                (true, false) => {
                    *next_step = false;
                    *error_flag = true;
                }
                (false, true) => {
                    *next_step = false;
                    *error_flag = false;
                }
                (false, false) => {
                    *next_step = false;
                    *next_step = false;
                }
            }
        }
    }
    Ok(())
}
