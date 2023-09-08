// =======================================================
pub mod common_elements;
mod init;
pub mod next;
pub mod rule_bytes;
pub mod rule_str;
pub mod runner;
pub mod traits;
use super::*;
use indexmap::IndexSet;
use log::error;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
// =======================================================a
pub use common_elements::*;

impl AsRef<str> for RegexRaw {
    fn as_ref(&self) -> &str {
        match self {
            RegexRaw::DefaultRegex(value) => value,
            RegexRaw::FancyRegex(value) => value,
            RegexRaw::BytesRegex(value) => value,
        }
    }
}
