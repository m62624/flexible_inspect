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
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "log_rust")]
use chrono::Local;
#[cfg(feature = "log_rust")]
use std::io::Write;
#[cfg(feature = "log_rust")]
use colored::*;
#[cfg(feature = "log_rust")]
use std::sync::Once;
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
            writeln!(
                buf,
                "[{} {} {}]↴\n{}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                match record.level() {
                    log::Level::Error => "ERROR".to_string().red(),
                    log::Level::Warn => "WARN".to_string().yellow(),
                    log::Level::Info => "INFO".to_string().blue(),
                    log::Level::Debug => "DEBUG".to_string().green(),
                    log::Level::Trace => "TRACE".to_string().purple(),
                },
                record.target().bright_black(),
                record.args().to_string().bright_cyan()
            )
        })
        .init();
    });
}
