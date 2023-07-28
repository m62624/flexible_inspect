use super::*;

impl<I> CartridgeBase<RuleBytes, I, &[u8]> for TakeCartridgeForAsync<RuleBytes>
where
    I: IntoIterator<Item = RuleBytes>,
{
    fn id(&self) -> i64 {
        self.id
    }

    fn message(&mut self) -> &mut String {
        &mut self.message
    }

    fn root_rule(&self) -> &RuleBytes {
        &self.root_rule
    }

    fn run(&mut self, data: &[u8]) -> NextStep {
        rules::runner::run::<RuleBytes, &[u8]>(
            &self.root_rule,
            CaptureData {
                text_for_capture: HashSet::from([data]),
                hashmap_for_error: Default::default(),
                counter_value: Default::default(),
            },
        )
    }
}
