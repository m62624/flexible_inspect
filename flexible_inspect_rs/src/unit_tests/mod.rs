mod tests_cartridges;
mod tests_error;
mod tests_message;
mod tests_next;
mod tests_rules;
#[cfg(feature = "serde")]
mod tests_serde;
mod tests_validate;
use crate::message::filling_message;
use std::collections::HashMap;
