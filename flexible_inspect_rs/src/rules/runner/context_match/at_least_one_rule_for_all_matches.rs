use super::*;

/// in this mode, at least one rule must be passed for all matches
pub fn at_least_one_rule_for_all_matches<'a, R, C>(
    // get a unique stack of one root, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<'a, C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a>,
{
    
    // if let Some(mut frame) = stack.pop_front() {}
    NextStep::Finish
}
