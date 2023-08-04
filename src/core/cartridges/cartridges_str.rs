use super::*;

impl CartridgeBase<Rule, &str> for Cartridge<Rule> {
    fn run(&self, data: &str) -> NextStep {
        rules::runner::run::<Rule, &str>(
            &self.root_rule,
            CaptureData {
                text_for_capture: HashSet::from([data]),
                hashmap_for_error: Default::default(),
                counter_value: Default::default(),
            },
        )
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}
