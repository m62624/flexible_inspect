use super::*;

impl CartridgeBase<RuleBytes, &[u8]> for Cartridge<RuleBytes> {
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

    /// Get the `error code`
    fn get_id(&self) -> i64 {
        self.id
    }

    /// Get an `error message` with data
    fn get_message(&self) -> &str {
        &self.message
    }
}
