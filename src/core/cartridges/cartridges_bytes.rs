use std::sync::Arc;

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

    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_message(&self) -> &str {
        &self.message
    }

    fn get_root_rule(&self) -> &RuleBytes {
        &self.root_rule
    }
}

impl CartridgeBase<RuleBytes, Arc<[u8]>> for Cartridge<RuleBytes> {
    fn run(&self, data: Arc<[u8]>) -> NextStep {
        rules::runner::run::<RuleBytes, &[u8]>(
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

    fn get_root_rule(&self) -> &RuleBytes {
        &self.root_rule
    }
}
