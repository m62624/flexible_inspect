use colored::*;
use log::{info, warn};
use std::{fmt::Debug, str::FromStr};

pub trait FromBytes<T> {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<T>;
    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<T>;
}

impl FromBytes<i32> for i32 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<i32> {
        if bytes.len() == 4 {
            let mut array_bytes: [u8; 4] = Default::default();
            array_bytes.copy_from_slice(&bytes);
            return Some(i32::from_be_bytes(array_bytes));
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `16`, value {}",
                    bytes.len(),
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<i32> {
        if bytes.len() == 4 {
            let mut array_bytes: [u8; 4] = Default::default();
            array_bytes.copy_from_slice(&bytes);
            return Some(i32::from_le_bytes(array_bytes));
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `16`, value {}",
                    bytes.len(),
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}

pub mod bytes_to_number {
    use super::*;

    /// Convert a collection of bytes to numbers and filter out errors
    pub fn from_utf8<T: FromStr + Copy + Debug + ToString>(bytes: &[u8]) -> Option<T> {
        if let Ok(value) = String::from_utf8(bytes.to_vec()) {
            info!(
                "{}",
                format!(
                    "(stage 1) bytes converted to a string, value {}",
                    value.yellow()
                )
            );
            if let Ok(value) = value.parse::<T>() {
                info!(
                    "{}",
                    format!(
                        "(stage 2) string converted to a number, value {}",
                        value.to_string().yellow()
                    )
                );
                Some(value)
            } else {
                warn!(
                    "{}",
                    format!(
                        "(stage 2) string cannot be converted to a number, the value {}",
                        value.yellow()
                    )
                );
                None
            }
        } else {
            warn!(
                "{}",
                format!(
                    "(step 1) bytes cannot be converted to a string, the value {}",
                    format!("{:?}", bytes).yellow()
                )
            );
            None
        }
    }

    // /// Create a native endian integer value from its representation as a byte array in big endian
    // pub fn from_be_bytes<T: FromStr + Debug>(bytes: &[u8]) -> Option<T> {
    //     info!(
    //         "(mode from_be_bytes) bytes converted to a string, value {}",
    //         format!("{:?}", bytes).yellow()
    //     );
    //     Some(T::from_be_bytes(*bytes))
    // }

    // /// Create a native endian integer value from its representation as a byte array in little endian
    // pub fn from_le_bytes<T: FromStr + Debug>(bytes: &[u8]) -> Option<u32> {
    //     info!(
    //         "(mode from_le_bytes) bytes converted to a string, value {}",
    //         format!("{:?}", bytes).yellow()
    //     );
    //     Some(T::from_le_bytes(*bytes))
    // }
}

#[test]
fn check_boundary_raw_array() {
    let bytes: [u8; 16] = [
        0x78, 0x56, 0x34, 0x12, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0x9A, 0xBC, 0xDE,
        0xF0,
    ];
    let invalid_bytes: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

    let value = i32::from_be_bytes_non_const(&invalid_bytes);
    dbg!(value);
}
