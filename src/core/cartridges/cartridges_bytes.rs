use super::*;

impl<I> CartridgeBase<RuleBytes, I, &[u8]> for TakeCartridgeForAsync<RuleBytes, I>
where
    I: IntoIterator<Item = RuleBytes>,
{
    fn id(&self) -> i64 {
        self.id
    }

    fn message(&mut self) -> &mut String {
        &mut self.message
    }

    fn rules(&self) -> &Option<I> {
        &self.rules
    }

    fn run(&mut self, data: &[u8]) -> NextStep {
        let root_rule = RuleBytes::new(":: ROOT_RULE ::", MatchRequirement::MustBeFound)
            .extend(std::mem::take(&mut self.rules).unwrap());
        rules::runner::run::<RuleBytes, &[u8]>(
            &root_rule,
            CaptureData {
                text_for_capture: HashSet::from([data]),
                hashmap_for_error: Default::default(),
                counter_value: Default::default(),
            },
        )
    }
}
