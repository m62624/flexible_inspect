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

    fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.root_rule = self.root_rule.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.root_rule = self.root_rule.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.root_rule = self
            .root_rule
            .mode_at_least_one_rule_for_at_least_one_match();
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
