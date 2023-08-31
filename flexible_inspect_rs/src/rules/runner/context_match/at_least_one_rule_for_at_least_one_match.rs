use super::*;

/// in this mode, at least one rule must be passed for at least on match
pub fn at_least_one_rule_for_at_least_one_match<'a, R, C>(
    // get a unique stack of one root, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<'a, C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a>,
{
    let mut temp_stack: Option<VecDeque<(&R::RuleType, CaptureData<C>)>> = Some(VecDeque::new());
    if let Some(mut frame) = stack.pop_front() {}
    NextStep::Finish
}
