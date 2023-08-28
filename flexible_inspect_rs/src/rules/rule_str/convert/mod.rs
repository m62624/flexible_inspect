use log::{info, warn};
use std::{fmt::Debug, str::FromStr};

/// Convert a collection of strings to numbers and filter out errors
#[cfg(feature = "in_development")]
pub fn convert_and_filter_collection<'a, C, T>(words: C) -> Vec<T>
where
    C: IntoIterator<Item = &'a str>,
    T: PartialOrd + FromStr + Copy + Debug,
{
    let result = words
        .into_iter()
        .filter_map(|word| {
            if let Ok(parsed_value) = word.parse::<T>() {
                info!("сonverted '{}' to {:?}.", word, parsed_value);
                Some(parsed_value)
            } else {
                warn!("skipped '{}' due to conversion error.", word);
                None
            }
        })
        .collect();
    result
}

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
