use super::{traits::*, *};

impl CartridgeBase<&[u8]> for Cartridge<RuleBytes> {
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

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}

impl CartridgeModifiers for Cartridge<RuleBytes> {
    type CartridgeType = Cartridge<RuleBytes>;

    fn any_r_for_any_m(&mut self) -> Self {
        self.root_rule = self.root_rule.any_r_for_any_m();
        std::mem::take(self)
    }
}

#[cfg(feature = "export_to_other_languages")]
impl CartridgeBase<Arc<[u8]>> for Cartridge<RuleBytes> {
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

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_message(&self) -> &str {
        &self.message
    }
}
