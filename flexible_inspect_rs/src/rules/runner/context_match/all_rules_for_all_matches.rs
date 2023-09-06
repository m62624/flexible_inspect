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
    if let Some(mut frame) = stack.pop_front() {}
    NextStep::Finish
}
