use super::*;

/// in this mode, a rule must be passed for each match
pub fn all_rules_for_all_matches<'a, R, C>(
    // get a unique stack of one root rule, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<'a, C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a>,
{
    let mut temp_stack: VecDeque<(&R::RuleType, CaptureData<C>)> = VecDeque::new();
    if let Some(mut frame) = stack.pop_front() {
        trace!(
            "deleted rule from unique stack: ({}, {})",
            frame.0.get_str().yellow(),
            format!("{:#?}", frame.0.get_requirement()).yellow()
        );
        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                // ============================= LOG =============================
                debug!(
                    "run subrules from the root rule `({}, {})`",
                    frame.0.get_str().yellow(),
                    format!("{:#?}", frame.0.get_requirement()).yellow()
                );
                // ===============================================================

                if let Some(simple_rules) = &frame.0.get_simple_rules() {
                    // count of how many times one rule has worked for different matches
                    let mut counter_of_each_rule = HashMap::new();
                    // which matches have already been processed in the rule
                    // is necessary so you don't have to go through them again in the second cycle.
                    let mut selected_text = HashMap::new();
                    // rules that have passed the selections for all matches
                    let mut selected_rules = HashSet::new();
                    /*
                    The first step is to get a RegexSet for each match, based on it,
                    we get those rules that will definitely work, then check their modifiers
                     */
                    for data in &frame.1.text_for_capture {
                        // we get the indexes of the rules that are in the RegexSet
                        for index in R::get_selected_rules(simple_rules.1, data) {
                            let rule_from_regexset = simple_rules.0.get_index(index).unwrap();
                            // ============================= LOG =============================
                            debug!(
                                "found the rule `({}, {})` (root rule `({}, {})`) from the `RegexSet` category\nfor data `{:#?}`",
                                rule_from_regexset.get_str().yellow(),
                                format!("{:#?}", rule_from_regexset.get_requirement()).yellow(),
                                frame.0.get_str().yellow(),
                                format!("{:#?}", frame.0.get_requirement()).yellow(),
                                data
                            );
                            // ===============================================================
                            let mut captures = R::find_captures(rule_from_regexset, data);
                            if let NextStep::Error(error) =
                                NextStep::next_or_finish_or_error(rule_from_regexset, &mut captures)
                            {
                                // ============================= LOG =============================
                                error!(
                                    "the rule `({}, {})` (root rule `({}, {})`)\nfailed condition\nfor data `{:#?}`",
                                    rule_from_regexset.get_str().yellow(),
                                    format!("{:#?}", rule_from_regexset.get_requirement()).yellow(),
                                    frame.0.get_str().yellow(),
                                    format!("{:#?}", frame.0.get_requirement()).yellow(),
                                    data
                                );
                                // ===============================================================
                                return NextStep::Error(error);
                            }
                            /*
                            For each rule, let's mark the data that has already been checked,
                            so that we can exclude it in the second cycle
                             */
                            selected_text
                                .entry(rule_from_regexset)
                                .or_insert_with(HashSet::new)
                                .insert(data);
                            /*
                            Since in this mode `rule` * `data`, where each rule should work for every match,
                            we check how many times one rule from regexset was passed for matches,
                            if a rule worked for all matches, we write it to an exception.
                             */
                            *counter_of_each_rule.entry(index).or_insert(0) += 1;
                            if counter_of_each_rule[&index] == frame.1.text_for_capture.len() {
                                selected_rules.insert(rule_from_regexset);
                                temp_stack.push_back((rule_from_regexset, captures));
                            }
                        }
                    }
                    // The second step, in this stage we go through those rules and matches that are not in `RegexSet`.
                    for data in &frame.1.text_for_capture {
                        // we go through all the simple rules
                        for rule in simple_rules.0 {
                            // So the first condition is that we exclude those rules
                            // that have already been processed in RegexSet.
                            // ( excluded only those who, for all the coincidence, have been successful )
                            if !selected_rules.contains(rule) {
                                // If this rule worked for several matches,
                                // but not for all of them, then we get those values that have already been processed
                                // and exclude them
                                if let Some(value) = selected_text.get(rule) {
                                    if !value.contains(data) {
                                        let mut captures = R::find_captures(rule, data);
                                        if let NextStep::Error(err) = not_in_regexset::<R, C>(
                                            frame.0,
                                            rule,
                                            data,
                                            &mut captures,
                                        ) {
                                            return NextStep::Error(err);
                                        }
                                        temp_stack.push_back((rule, captures));
                                    }
                                } else {
                                    let mut captures = R::find_captures(rule, data);
                                    // If there were no successful matches in this rule,
                                    // it means that this is the first time
                                    // this rule has been run for validation
                                    if let NextStep::Error(err) =
                                        not_in_regexset::<R, C>(frame.0, rule, data, &mut captures)
                                    {
                                        return NextStep::Error(err);
                                    }
                                    temp_stack.push_back((rule, captures));
                                }
                            }
                        }
                    }
                }
                // The hird step, bypass the rules with the Lookahead and Lookbehind regex.
                if let Some(complex_rules) = frame.0.get_complex_rules() {
                    for data in &frame.1.text_for_capture {
                        for cmplx_rule in complex_rules {
                            let mut captures = R::find_captures(cmplx_rule, data);
                            if let NextStep::Error(err) =
                                NextStep::next_or_finish_or_error(cmplx_rule, &mut captures)
                            {
                                // ============================= LOG =============================
                                error!(
                                    "the rule `({}, {})` (root rule `({}, {})`) \nfailed condition\nfor data `{:#?}`",
                                    cmplx_rule.get_str().yellow(),
                                    format!("{:#?}", cmplx_rule.get_requirement()).yellow(),
                                    frame.0.get_str().yellow(),
                                    format!("{:#?}", frame.0.get_requirement()).yellow(),
                                    data
                                );
                                // ===============================================================
                                return NextStep::Error(err);
                            }
                            // ============================= LOG =============================
                            debug!(
                                "found the rule `({}, {})` (root rule `({}, {})`) from the `Complex Rule` category\nfor data `{:#?}`",
                                cmplx_rule.get_str().yellow(),
                                format!("{:#?}", cmplx_rule.get_requirement()).yellow(),
                                frame.0.get_str().yellow(),
                                format!("{:#?}", frame.0.get_requirement()).yellow(),
                                data);
                            // ===============================================================
                            temp_stack.push_back((cmplx_rule, captures));
                        }
                    }
                }
            }
            NextStep::Finish => {
                // ============================= LOG =============================
                debug!(
                    "the rule `({}, {})` is finished, the result is `Ok`",
                    frame.0.get_str().yellow(),
                    format!("{:#?}", frame.0.get_requirement()).yellow(),
                );
                // ===============================================================
            }
            NextStep::Error(err) => {
                // ================= (LOG) =================
                error!(
                    "the rule `({}, {})` didn't work",
                    frame.0.get_str().yellow(),
                    format!("{:#?}", frame.0.get_requirement()).yellow(),
                );
                // =========================================
                return NextStep::Error(err);
            }
        }
    }
    stack.extend(temp_stack.drain(..));
    // ================= (LOG) =================
    info!("for all matches all rules worked successfully");
    // =========================================
    NextStep::Finish
}

// Function for checking rules not included in `RegexSet`.
fn not_in_regexset<'a, R, C>(
    root_rule: &R::RuleType,
    rule: &R::RuleType,
    data: &C,
    captures: &mut CaptureData<'a, C>,
) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: IntoSpecificCaptureType<'a>,
{
    // ============================= LOG =============================
    debug!(
        "the rule `({}, {})` (root rule `({},{})`) isn't in the `RegexSet` category\nfor data `{:#?}`",
        rule.get_str().yellow(),
        format!("{:#?}", rule.get_requirement()).yellow(),
        root_rule.get_str().yellow(),
        format!("{:#?}", root_rule.get_requirement()).yellow(),
        data
    );
    // ===============================================================

    if let NextStep::Error(error) = NextStep::next_or_finish_or_error(rule, captures) {
        // ============================= LOG =============================
        error!(
            "the rule `({}, {})` (root rule `({},{})`)\nfailed condition\nfor data `{:#?}`",
            rule.get_str().yellow(),
            format!("{:#?}", rule.get_requirement()).yellow(),
            root_rule.get_str().yellow(),
            format!("{:#?}", root_rule.get_requirement()).yellow(),
            data
        );
        return NextStep::Error(error);
        // ===============================================================
    }
    NextStep::Finish
}
