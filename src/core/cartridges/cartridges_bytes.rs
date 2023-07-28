use super::*;

impl CartridgeBase<RuleBytes, &[u8]> for Cartridge<RuleBytes> {
    fn id(&self) -> i64 {
        self.id
    }

    fn message(&self) -> &mut String {
        todo!()
    }

    fn root_rule(&self) -> &RuleBytes {
        &self.root_rule
    }

    fn run(&self, data: &[u8]) -> NextStep {
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
