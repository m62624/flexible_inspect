use super::*;

impl<I> CartridgeBase<Rule, I, &str> for TakeCartridgeForAsync<Rule, I>
where
    I: IntoIterator<Item = Rule> + Default,
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

    fn run(&mut self, data: &str) -> NextStep {
        let root_rule = Rule::new(":: ROOT_RULE ::", MatchRequirement::MustBeFound)
            .extend(std::mem::take(&mut self.rules).unwrap());
        rules::runner::run::<Rule, &str>(
            &root_rule,
            CaptureData {
                text_for_capture: HashSet::from([data]),
                hashmap_for_error: Default::default(),
                counter_value: Default::default(),
            },
        )
    }
}
