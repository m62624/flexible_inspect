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
// =====================================================================
static INIT: Once = Once::new();
// =====================================================================

#[cfg(not(tarpaulin_include))]
fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            let level_colored = match record.level() {
                log::Level::Error => format!("ERROR"),
                log::Level::Warn => format!("WARN"),
                log::Level::Info => format!("INFO"),
                log::Level::Debug => format!("DEBUG"),
                log::Level::Trace => format!("TRACE"),
            };
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                level_colored,
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
/// # Arguments
/// * `log_level` - log level
/// LEVELS: OFF, ERROR, INFO, DEBUG, TRACE
#[cfg(not(tarpaulin_include))]
#[wasm_bindgen]
pub fn init_logger(log_level: String) {
    // env_logger is called only once
    INIT.call_once(|| {
        let log_level = log_level.to_uppercase();
        let logger = if log_level == "TRACE" {
            setup_logger(LevelFilter::Trace)
        } else if log_level == "DEBUG" {
            setup_logger(LevelFilter::Debug)
        } else if log_level == "INFO" {
            setup_logger(LevelFilter::Info)
        } else if log_level == "WARN" {
            setup_logger(LevelFilter::Warn)
        } else if log_level == "ERROR" {
            setup_logger(LevelFilter::Error)
        } else {
            setup_logger(LevelFilter::Off)
        };
        logger.unwrap_or_else(|_| {
            console_error_panic_hook::set_once();
            panic!("Error initializing logger");
        });
    });
}
