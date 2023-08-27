use super::*;

pub fn number_range_status<R: RuleBase, C: PartialEq + Eq + Hash>(
    rule: &R,
    captures: &mut CaptureData<C>,
) -> NextStep {

    
    NextStep::Finish
}
