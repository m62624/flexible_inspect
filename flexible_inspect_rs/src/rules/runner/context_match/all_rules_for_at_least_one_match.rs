use super::*;

/// in this mode all rules must be passed for at least one match
pub fn all_rules_for_at_least_one_match<'a, R, C>(
    // get a unique stack of one root, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<'a, C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a>,
{
    // let mut temp_stack: Stack<'a, R, C> = VecDeque::new();
    // if let Some(mut frame) = stack.pop_front() {}
    NextStep::Finish
}
