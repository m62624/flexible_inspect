use super::*;
use std::collections::VecDeque;

pub mod sync_fn {
    use super::*;
    /// Запускаем `runner`, на входе получает по одному корню, идет проверка до тех пор
    /// пока `rule` не окажется конечным (из корня по каждому подправилу и т.д)
    //       | root | - > subrules -> ( ∞ | last element )
    //       \  |  /
    //          |
    //         \|/

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
                            rule::Rule::make_sub_regex_set(rule.get_subrules(), text)
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
    }
}
pub mod async_fn {}

/// Разрешение на проверки `Rules` в глубину
mod step_by_step {
    use super::*;
    /*
    - MustBeFound - Если найдено и нет subrules -> Ok()
    - MustBeFound -> Если Найдено и есть subrules -> continue
      |
      | --> MustBeFound - Если не найдено и есть subrules -> skip subrules | Error()
     \|/
    - MustBeFound - Если не найдено и нет subrules -> Error()
    ========================================================================
    - MustNotBeFound - Если Найдено и нет subrules -> error()
    - MustNotBeFound - Если Найдено и есть subrules -> continue
       |
       | ---> MustNotBeFound - Если не найдено и есть subrules -> skip subrules | Ok()
      \|/
    - MustNotBeFound - Не найдено и нет subrules -> Ok()
     */
    pub fn result_on_the_match(
        rule: &rule::Rule,
        captures: bool,
        error_flag: &mut bool,
        next_step: &mut bool,
    ) -> PyResult<()> {
        match rule.get_requirement()? {
            rule::MatchRequirement::MustBeFound => {
                match (captures, rule.get_subrules().is_some()) {
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
                }
            }
            rule::MatchRequirement::MustNotBefound => {
                match (captures, rule.get_subrules().is_some()) {
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
}
