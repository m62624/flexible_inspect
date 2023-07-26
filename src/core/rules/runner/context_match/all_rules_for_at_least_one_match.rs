use log::{debug, error, info, trace};

use super::*;
use crate::core::rules::traits::RuleBase;
use crate::core::rules::{next::NextStep, traits::CalculateValueRules, CaptureData};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

/// in this mode all rules must be passed for at least one match
pub fn all_rules_for_at_least_one_match<'a, R, C>(
    // this parameter is required for logs
    rule_ref: &R::RuleType,
    // get a unique stack of one root cmplx_rule, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: PartialEq + Eq + Hash + Debug,
{
    let mut temp_stack: VecDeque<(&R::RuleType, CaptureData<C>)> = VecDeque::new();
    // ============================= LOG =============================
    debug!("the local rule stack `{}` is received", {
        rule_ref.get_str()
    });
    // ===============================================================
    while let Some(mut frame) = stack.pop_front() {
        // ============================= LOG =============================
        debug!(
            "\ncheck the state of the rule `({}, {:#?})` \nfrom the local stack `{}`",
            frame.0.get_str(),
            frame.0.get_requirement(),
            rule_ref.get_str()
        );
        // ===============================================================
        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                // Хранит ошибку, если она есть
                let mut err: Option<HashMap<String, String>> = None;
                // Статус, что нашли одно совпадение на которое сработали все правила
                let mut rule_matched_for_any_text = false;

                'skip_data: for data in &frame.1.text_for_capture {
                    if let Some(simple_rules) = frame.0.get_simple_rules() {
                        let mut selected_rules = HashSet::new();
                        for index in R::get_selected_rules(simple_rules.1, data) {
                            let rule_from_regexset = simple_rules.0.get_index(index).unwrap();
                            // ============================= LOG =============================
                            trace!(
                                "found `({}, {:#?})` rule from `RegexSet` for `{:#?}` data",
                                rule_from_regexset.get_str(),
                                rule_from_regexset.get_requirement(),
                                data
                            );
                            // ===============================================================
                            let mut captures = R::find_captures(rule_from_regexset, data);
                            if let NextStep::Error(error) =
                                NextStep::next_or_finish_or_error(rule_from_regexset, &mut captures)
                            {
                                // ============================= LOG =============================
                                error!(
                                    "the rule `{}` failed condition for data `{:#?}`",
                                    rule_from_regexset.get_str(),
                                    data
                                );
                                // ===============================================================
                                err = error;
                                continue 'skip_data;
                            }
                            selected_rules.insert(rule_from_regexset);
                            temp_stack.push_back((rule_from_regexset, captures));
                        }

                        for rule in simple_rules.0 {
                            if !selected_rules.contains(rule) {
                                let mut captures = R::find_captures(rule, data);
                                if let NextStep::Error(error) =
                                    NextStep::next_or_finish_or_error(rule, &mut captures)
                                {
                                    err = error;
                                    continue 'skip_data;
                                }
                                temp_stack.push_back((rule, captures));
                            }
                        }
                    }
                    if let Some(cmpl_rules) = frame.0.get_complex_rules() {
                        for cmplx_rule in cmpl_rules {
                            // ============================= LOG =============================
                            trace!(
                                "the rule `({}, {:#?})` from `complex_rules`",
                                cmplx_rule.get_str(),
                                cmplx_rule.get_requirement()
                            );
                            // ===============================================================
                            let mut captures = R::find_captures(cmplx_rule, data);
                            if let NextStep::Error(error) =
                                NextStep::next_or_finish_or_error(cmplx_rule, &mut captures)
                            {
                                err = error;
                                continue 'skip_data;
                            }
                            temp_stack.push_back((cmplx_rule, captures));
                        }
                    }
                    info!("all rules passed successfully for the text `{:#?}` ", data);
                    // Если дошли до конца цикла (в рамках одного элемента), значит все правила сработали
                    rule_matched_for_any_text = true;
                    break;
                }
                if rule_matched_for_any_text {
                    // Финальный этап, мы загружаем всё в`stack` для дальнейшей обработки
                    stack.extend(temp_stack.drain(..));
                } else {
                    // ================= (LOG) =================
                    error!("all of the rules do not match any text");

                    // =========================================
                    return NextStep::Error(err);
                }
            }
            NextStep::Finish => (),
            NextStep::Error(err) => {
                // ================= (LOG) =================
                error!("all of the rules do not match any text");

                // =========================================
                return NextStep::Error(err);
            }
        }
    }
    NextStep::Finish
}
