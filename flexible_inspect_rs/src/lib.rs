#![warn(missing_docs)]
//! The `Flexible_inspect` is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\
//! The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.

//! # Implementation
//! Implementation: Regex a formal language used in computer programs working with text to find and manipulate substrings in text,
//! based on the use of metacharacters It provides powerful pattern-based text search and manipulation capabilities.
//! On the other hand, `flexible_inspect` is a specialized library for data validation.
//! It's designed specifically for easy and convenient validation of data with varying degrees of nesting

//! # Abstraction Level
//! Abstraction Level: Regex works at the level of regular expressions, which are sequences of characters that define a pattern.
//! It has a wide set of functions and operators to define complex patterns for text search and replacement. `flexible_inspect` , on the other hand, provides a higher level of abstraction for data validation.
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
    use chrono::Local;
    use colored::*;
    use std::io::Write;
    use std::sync::Once;

    // =====================================================================
    /// For one-time initialization to the logger
    static INIT: Once = Once::new();
    // =====================================================================
    /// Initialization of the logger
    #[cfg(not(tarpaulin_include))]
    pub(crate) fn init_logger() {
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
                        log::Level::Error => "ERROR".red(),
                        log::Level::Warn => "WARN".yellow(),
                        log::Level::Info => "INFO".blue(),
                        log::Level::Debug => "DEBUG".green(),
                        log::Level::Trace => "TRACE".purple(),
                    },
                    record.target().bright_black(),
                    record.args().to_string().bright_cyan()
                )
            })
            .init();
        });
    }
}
