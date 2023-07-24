use super::*;
use std::collections::{HashMap, HashSet};
/// A structure that stores all the data for processing the capture
#[derive(Debug)]
pub struct CaptureData<'s> {
    pub text_for_capture: HashSet<CaptureType<'s>>,
    pub hashmap_for_error: HashMap<String, String>,
    pub counter_value: usize,
}

/// A structure that stores the type of capture
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CaptureType<'s> {
    Str(&'s str),
    Bytes(&'s [u8]),
}
impl<'a> CaptureData<'a> {
    pub fn is_some(&self) -> bool {
        !self.text_for_capture.is_empty()
    }
}
