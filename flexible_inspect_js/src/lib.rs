//! The `Flexible_inspect` is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\
//! The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.

mod cartridges;
mod error;
mod rules;
mod template_validator;
use chrono::prelude::*;
use flexible_inspect_rs::prelude::*;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::sync::Once;
use wasm_bindgen::prelude::*;
#[cfg(test)]
mod unit_tests;

pub use cartridges::{cartridge_bytes::WasmCartridgeBytes, cartridge_str::WasmCartridge};
pub use error::*;
pub use rules::{rule_bytes::WasmRuleBytes, rule_str::WasmRule, WasmMatchRequirement};
pub use template_validator::{
    validate_bytes::WasmTemplateValidatorBytes, validate_str::WasmTemplateValidator,
};
// =====================================================================
static INIT: Once = Once::new();
// =====================================================================

/// Log levels for logging support
#[wasm_bindgen]
pub enum LogLevel {
    /// * `ERROR` - only errors are displayed
    /// > Includes all messages that did not pass the rules condition test
    ERROR,
    /// * `INFO` - errors and information are displayed
    /// > Includes all messages that did not pass the rules condition test and information about the rules that were caught
    INFO,
    /// * `DEBUG` - errors, information and debug information are displayed
    /// > Includes all messages that did not pass the rules condition test, information about the rules that were caught and debug information about debug what modifiers are applied
    DEBUG,
    /// * `TRACE` - errors, information, debug information and trace information are displayed
    /// > Includes all messages that did not pass the rules condition test, information about the rules that were caught, debug information about debug what modifiers are applied and trace information about what type of regular expression is used, and whether it is included in the [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html).
    TRACE,
}

#[cfg(not(tarpaulin_include))]
fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}]↴\n{}\n",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(fern::Output::call(console_log::log))
        .apply()?;
    Ok(())
}

/// Initialization of the logger (no color support)
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
/// ## Example:
/// ```js
/// init_logger(LogLevel.Debug);
/// ```
#[cfg(not(tarpaulin_include))]
#[wasm_bindgen]
pub fn init_logger(log_level: LogLevel) {
    // env_logger is called only once
    INIT.call_once(|| {
        let log_level = match log_level {
            LogLevel::ERROR => LevelFilter::Error,
            LogLevel::INFO => LevelFilter::Info,
            LogLevel::DEBUG => LevelFilter::Debug,
            LogLevel::TRACE => LevelFilter::Trace,
        };
        setup_logger(log_level).unwrap_or_else(|_| {
            console_error_panic_hook::set_once();
            panic!("Error initializing logger");
        });
    });
}
