#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
//! The `Flexible_inspect` is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\
//! The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.

//! # Implementation
//! Implementation: Regex a formal language used in computer programs working with text to find and manipulate substrings in text,
//! based on the use of metacharacters It provides powerful pattern-based text search and manipulation capabilities.
//! On the other hand, `flexible_inspect` is a specialized library for data validation.
//! It's designed specifically for easy and convenient validation of data with varying degrees of nesting

//! # Abstraction Level
//! `flexible_inspect` provides a higher level of abstraction for data validation.
//! It has a simple way for different nesting of regular expressions and also has the ability to change data validation modes

//! # Ease of use
//! Designed with an emphasis on ease of use. Offers a simple and intuitive `API` that makes it easy to define validation rules and validate data.
//! With a higher level of abstraction and the use of finite automata, `flexible_inspect` simplifies the data validation process and reduces the amount of code required.

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
// =====================================================================
/// Everything you need to get started with the validator
pub mod prelude {
    pub use crate::cartridges::{traits::CartridgeModifiers, Cartridge};
    pub use crate::error::ValidationError;
    pub use crate::rules::MatchRequirement;
    pub use crate::rules::{rule_bytes::RuleBytes, rule_str::Rule, traits::RuleModifiers};
    pub use crate::template_validator::{TemplateValidator, ValidatorBase};
}

/// Module with error messages
#[cfg(feature = "export_to_other_languages")]
pub mod error_messages {
    /// Error message when the body of `Rule` is missing
    pub const ERR_OPTION_RULE: &str = "\nThe body of `Rule` is missing (inside `Rule` is the value `None`), you may have used modifiers separately from initializations, they take the value (ownership) of `Rule` (std::mem::take) and return the already modified version (specify the modifier in the same place where you initialize `Rule`).\nOr you have specified a nested `Rule` for `RuleBytes`\n";

    /// Error message when the body of `RuleBytes` is missing
    pub const ERR_OPTION_RULE_BYTES: &str = "\nThe body of `RuleBytes` is missing (inside `RuleBytes` is the value `None`), you may have used modifiers separately from initializations, they take the value (ownership) of `RuleBytes` (std::mem::take) and return the already modified version (specify the modifier in the same place where you initialize `RuleBytes`).\nOr you have specified a nested `RuleBytes` for `Rule`\n";

    /// Error message when the body of `Cartridge` is missing
    pub const ERR_OPTION_CARTRIDGE: &str = "\nThe body of `Cartridge` is missing (inside `Cartridge` is the value `None`), you may have used modifiers separately from initializations, they take the value (ownership) of `Cartridge` (std::mem::take) and return the already modified version (specify the modifier in the same place where you initialize `Cartridge`).\nOr you have specified a root `RuleBytes` for `Cartridge`";

    /// Error message when the body of `CartridgeBytes` is missing
    pub const ERR_OPTION_CARTRIDGE_BYTES: &str = "\nThe body of `CartridgeBytes` is missing (inside `CartridgeBytes` is the value `None`), you may have used modifiers separately from initializations, they take the value (ownership) of `CartridgeBytes` (std::mem::take) and return the already modified version (specify the modifier in the same place where you initialize `CartridgeBytes`).\nOr you have specified a nested `Rule` for `CartridgeBytes`";
}

/// # Logs
/// Includes color-coded logging support (**do not need to be declared in the code**)
///
/// **Recommendations:** \
/// Use logging to find out what regular expression is caught,
/// whether matches should be found or not, what mode the cartridges are running in, etc.\
/// To enable logging support it is necessary to specify the `FLEX_VALIDATOR_LOG` environment variable before running the file
/// Levels of logging support:
/// * `ERROR` - only errors are displayed
/// > Includes all messages that did not pass the rules condition test
/// * `INFO` - errors and information are displayed
/// > Includes all messages that did not pass the rules condition test and information about the rules that were caught
/// * `DEBUG` - errors, information and debug information are displayed
/// > Includes all messages that did not pass the rules condition test, information about the rules that were caught and debug information about debug what modifiers are applied
/// * `TRACE` - errors, information, debug information and trace information are displayed
/// > Includes all messages that did not pass the rules condition test, information about the rules that were caught, debug information about debug what modifiers are applied and trace information about what type of regular expression is used, and whether it is included in the [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html).
/// # Example:
/// ```bash
/// FLEX_VALIDATOR_LOG=INFO cargo run
/// ```

#[cfg(feature = "log_rust")]
pub mod logs {
    const DATE_FORMAT_STR: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";
    use super::*;
    use std::io::Write;
    use std::sync::Once;
    use time::{format_description, OffsetDateTime};
    // =====================================================================
    /// For one-time initialization to the logger
    static INIT: Once = Once::new();
    // =====================================================================
    /// Initialization with time zone offset.\
    /// Function does not have to declare the logging work.
    /// It is necessary for logging time offset.
    /// `hour_offset` - offset from UTC time ( *to offset the time, initialize earlier than the rules* )
    /// To run logging, specify an environment variable with the desired logging level
    /// ```bash
    /// FLEX_VALIDATOR_LOG=DEBUG "command to run the code"
    /// ```
    pub fn init_logger_with_offset(hour_offset: i32) {
        INIT.call_once(|| {
            env_logger::Builder::from_env(
                env_logger::Env::new().filter_or("FLEX_VALIDATOR_LOG", "OFF"),
            )
            .format(move |buf, record| {
                writeln!(
                    buf,
                    "{} [{} {}]↴\n{}\n",
                    match record.level() {
                        log::Level::Error => "ERROR".red(),
                        log::Level::Warn => "WARN".yellow(),
                        log::Level::Info => "INFO".blue(),
                        log::Level::Debug => "DEBUG".cyan(),
                        log::Level::Trace => "TRACE".purple(),
                    },
                    OffsetDateTime::now_local()
                        .unwrap_or_else(|_| {
                            OffsetDateTime::now_utc() + time::Duration::hours(hour_offset as i64)
                        })
                        .format(
                            &format_description::parse(DATE_FORMAT_STR)
                                .expect("invalid time format")
                        )
                        .expect("invalid time format"),
                    record.target().bright_black(),
                    record.args().to_string().bright_green()
                )
            })
            .init();
        });
    }
}
