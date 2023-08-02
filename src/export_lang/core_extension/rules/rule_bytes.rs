use super::*;

impl CartridgeBase<RuleBytes, Arc<[u8]>> for Cartridge<RuleBytes> {
    fn run(&self, data: Arc<[u8]>) -> NextStep {
        crate::core::rules::runner::run::<RuleBytes, &[u8]>(
            &self.root_rule,
            CaptureData {
                text_for_capture: HashSet::from([data.as_ref()]),
                hashmap_for_error: Default::default(),
                counter_value: Default::default(),
            },
        )
    }

    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_message(&self) -> &str {
        &self.message
    }

}
