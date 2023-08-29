use log::{info, warn};
use std::{fmt::Debug, str::FromStr};

/// Convert a string to a number and filter out errors
pub fn convert_and_filter<T>(word: &str) -> Option<T>
where
    T: PartialOrd + FromStr + Copy + Debug,
{
    if let Ok(parsed_value) = word.parse::<T>() {
        info!("converted '{}' to {:?}.", word, parsed_value);
        Some(parsed_value)
    } else {
        warn!("skipped '{}' due to conversion error.", word);
        None
    }
}
