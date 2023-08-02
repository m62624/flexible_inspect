// =======================================================
pub mod base_error;
pub mod cartridges;
pub mod message;
pub mod rules;
pub mod validator;
#[cfg(any(feature = "serde", feature = "wasm"))]
use serde::{Deserialize, Serialize, Serializer,Deserializer};
// =======================================================

const ERR_OPTION: &str = " The body of `Rule` is missing, maybe you used modifiers, they borrow `Rule`, modifiers modify it and return the already modified version";

pub const DEFAULT_CAPTURE: &str = "main_capture";
