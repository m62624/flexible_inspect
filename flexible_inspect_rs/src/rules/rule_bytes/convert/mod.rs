use colored::*;
use log::{info, warn};
use std::{fmt::Debug, str::FromStr};

// ======================================================================================================
const I8_LEN: usize = 1;
const I16_LEN: usize = 2;
const I32_LEN: usize = 4;
const I64_LEN: usize = 8;
const I128_LEN: usize = 16;
const F32_LEN: usize = 4;
const F64_LEN: usize = 8;
// ======================================================================================================

// ======================================================================================================
pub trait FromBytes<T: FromStr + Copy + Debug + ToString> {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<T>;
    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<T>;
    fn from_utf8(bytes: &[u8]) -> Option<T> {
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
}
// ======================================================================================================

impl FromBytes<i8> for i8 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<i8> {
        if bytes.len() == I8_LEN {
            info!("conversion is possible for `i8`, value {:?}", bytes);
            let mut array_bytes: [u8; I8_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i8::from_be_bytes(array_bytes);
            info!("conversion result `i8`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I8_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<i8> {
        if bytes.len() == I8_LEN {
            info!("conversion is possible for `i8`, value {:?}", bytes);
            let mut array_bytes: [u8; I8_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i8::from_le_bytes(array_bytes);
            info!("conversion result `i8`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I8_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}

impl FromBytes<i16> for i16 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<i16> {
        if bytes.len() == I16_LEN {
            info!("conversion is possible for `i16`, value {:?}", bytes);
            let mut array_bytes: [u8; I16_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i16::from_be_bytes(array_bytes);
            info!("conversion result `i16`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I16_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<i16> {
        if bytes.len() == I16_LEN {
            info!("conversion is possible for `i16`, value {:?}", bytes);
            let mut array_bytes: [u8; I16_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i16::from_le_bytes(array_bytes);
            info!("conversion result `i16`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I16_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}

impl FromBytes<i32> for i32 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<i32> {
        if bytes.len() == I32_LEN {
            info!("conversion is possible for `i32`, value {:?}", bytes);
            let mut array_bytes: [u8; I32_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i32::from_be_bytes(array_bytes);
            info!("conversion result `i32`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I32_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<i32> {
        if bytes.len() == I32_LEN {
            info!("conversion is possible for `i32`, value {:?}", bytes);
            let mut array_bytes: [u8; I32_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i32::from_le_bytes(array_bytes);
            info!("conversion result `i32`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I32_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}

impl FromBytes<i64> for i64 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<i64> {
        if bytes.len() == I64_LEN {
            info!("conversion is possible for `i64`, value {:?}", bytes);
            let mut array_bytes: [u8; I64_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i64::from_be_bytes(array_bytes);
            info!("conversion result `i64`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I64_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<i64> {
        if bytes.len() == I64_LEN {
            info!("conversion is possible for `i64`, value {:?}", bytes);
            let mut array_bytes: [u8; I64_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i64::from_le_bytes(array_bytes);
            info!("conversion result `i64`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I64_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}

impl FromBytes<i128> for i128 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<i128> {
        if bytes.len() == I128_LEN {
            info!("conversion is possible for `i128`, value {:?}", bytes);
            let mut array_bytes: [u8; I128_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i128::from_be_bytes(array_bytes);
            info!("conversion result `i128`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I128_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<i128> {
        if bytes.len() == I128_LEN {
            info!("conversion is possible for `i128`, value {:?}", bytes);
            let mut array_bytes: [u8; I128_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = i128::from_le_bytes(array_bytes);
            info!("conversion result `i128`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    I128_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}

impl FromBytes<f32> for f32 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<f32> {
        if bytes.len() == F32_LEN {
            info!("conversion is possible for `f32`, value {:?}", bytes);
            let mut array_bytes: [u8; F32_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = f32::from_be_bytes(array_bytes);
            info!("conversion result `f32`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    F32_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<f32> {
        if bytes.len() == F32_LEN {
            info!("conversion is possible for `f32`, value {:?}", bytes);
            let mut array_bytes: [u8; F32_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = f32::from_le_bytes(array_bytes);
            info!("conversion result `f32`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    F32_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}

impl FromBytes<f64> for f64 {
    fn from_be_bytes_non_const(bytes: &[u8]) -> Option<f64> {
        if bytes.len() == F64_LEN {
            info!("conversion is possible for `f64`, value {:?}", bytes);
            let mut array_bytes: [u8; F64_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = f64::from_be_bytes(array_bytes);
            info!("conversion result `f64`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    F64_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }

    fn from_le_bytes_non_const(bytes: &[u8]) -> Option<f64> {
        if bytes.len() == F64_LEN {
            info!("conversion is possible for `f64`, value {:?}", bytes);
            let mut array_bytes: [u8; F64_LEN] = Default::default();
            array_bytes.copy_from_slice(bytes);
            let result = f64::from_le_bytes(array_bytes);
            info!("conversion result `f64`, value {:?}", result);
            return Some(result);
        } else {
            warn!(
                "{}",
                format!(
                    "length of array `{}`, for conversions must be `{}`, value {}",
                    bytes.len(),
                    F64_LEN,
                    format!("{:?}", bytes).yellow()
                )
            );
        }
        None
    }
}
