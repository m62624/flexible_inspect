// =======================================================
pub mod base_error;
pub mod cartridges;
pub mod message;
pub mod rules;
pub mod validator;
// =======================================================
#[cfg(any(feature = "serde", feature = "wasm"))]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// This is error message when `Rule` is empty
const ERR_OPTION: &str = " The body of `Rule` is missing, maybe you used modifiers, they borrow `Rule`, modifiers modify it and return the already modified version";

/// This is reserved standard value for error filling
pub const DEFAULT_CAPTURE: &str = "main_capture";
