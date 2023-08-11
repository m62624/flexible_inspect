//! The Data validator is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\
//! The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.

// =====================================================================
mod cartridges;
mod error;
mod message;
mod rules;
mod template_validator;
#[cfg(test)]
mod unit_tests;
// =====================================================================
use colored::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::Write;
#[cfg(feature = "log_rust")]
use std::sync::Once;
use chrono::Local;
// =====================================================================
pub mod prelude {
    pub use crate::cartridges::{traits::CartridgeModifiers, Cartridge};
    pub use crate::error::traits::ValidationError;
    pub use crate::rules::MatchRequirement;
    pub use crate::rules::{rule_bytes::RuleBytes, rule_str::Rule, traits::RuleModifiers};
    pub use crate::template_validator::{TemplateValidator, ValidatorBase};
    pub use futures::StreamExt;
}

// =====================================================================
/// For one-time initialization to the logger
#[cfg(feature = "log_rust")]
static INIT: Once = Once::new();
// =====================================================================
/// Initialization of the logger
#[cfg(not(tarpaulin_include))]
#[cfg(feature = "log_rust")]
fn init_logger() {
    // env_logger is called only once
    INIT.call_once(|| {
        env_logger::Builder::from_env(
            env_logger::Env::new().filter_or("FLEX_VALIDATOR_LOG", "OFF"),
        )
        .format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(
                buf,
                "[{} {} {}] {}",
                timestamp,
                match record.level() {
                    log::Level::Error => format!("ERROR").red(),
                    log::Level::Warn => format!("WARN").yellow(),
                    log::Level::Info => format!("INFO").blue(),
                    log::Level::Debug => format!("DEBUG").green(),
                    log::Level::Trace => format!("TRACE").purple(),
                },
                record.target().bright_black(),
                record.args().to_string().bright_cyan()
            )
        })
        .init();
    });
}
