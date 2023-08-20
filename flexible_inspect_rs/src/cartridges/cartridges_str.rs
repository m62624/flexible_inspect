use indexmap::IndexSet;

use super::{traits::*, *};

impl CartridgeBase<&str> for Cartridge<Rule> {
    fn run(&self, data: &str) -> NextStep {
        rules::runner::run::<Rule, &str>(
            &self.root_rule,
            CaptureData {
                text_for_capture: IndexSet::from([data]),
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

impl CartridgeModifiers for Cartridge<Rule> {
    type CartridgeType = Cartridge<Rule>;

    fn any_r_for_any_m(mut self) -> Self {
        self.root_rule = self.root_rule.any_r_for_any_m();
        self
    }
}

#[cfg(feature = "export_to_other_languages")]
impl CartridgeBase<Arc<str>> for Cartridge<Rule> {
    fn run(&self, data: Arc<str>) -> NextStep {
        rules::runner::run::<Rule, &str>(
            &self.root_rule,
            CaptureData {
                text_for_capture: IndexSet::from([data.as_ref()]),
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
