use super::*;
mod tests_cartridges;
mod tests_message;
mod tests_rules;
#[cfg(feature = "serde")]
mod tests_serde;
use crate::core::message::filling_message;
use std::collections::HashMap;
