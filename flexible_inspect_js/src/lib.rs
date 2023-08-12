//! The Data validator is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\
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

#[wasm_bindgen]
pub enum LogLevel {
    ERROR,
    INFO,
    DEBUG,
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

/// Initialization of the logger
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
