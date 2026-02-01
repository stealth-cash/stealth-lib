//! Legacy utilities module for backwards compatibility.
//!
//! # Deprecated
//!
//! This module is deprecated. Use [`crate::error`] and [`crate::encoding`] instead.
//!
//! ## Migration
//!
//! ```ignore
//! // Old code:
//! use stealth_lib::utils::SolanaError;
//!
//! // New code:
//! use stealth_lib::Error;
//! ```

#![allow(missing_docs)]
#![allow(deprecated)]

use std::fmt::Display;

/// Legacy error type.
///
/// # Deprecated
///
/// Use [`crate::Error`] instead.
#[deprecated(since = "1.0.0", note = "Use crate::Error instead")]
#[derive(Debug, PartialEq)]
pub struct SolanaError {
    error_msg: String,
    error_name: String,
    #[allow(unused)]
    error_code_number: u32,
    #[allow(unused)]
    error_origin: Option<String>,
    #[allow(unused)]
    compared_values: Option<String>
}

impl Display for SolanaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {} - {}", self.error_name, self.error_msg)
    }
}

impl std::error::Error for SolanaError {}

/// Creates a legacy error.
///
/// # Deprecated
///
/// Use [`crate::Error`] variants instead.
#[deprecated(since = "1.0.0", note = "Use crate::Error variants instead")]
pub fn err(msg: &str) -> SolanaError {
    SolanaError {
        error_msg: msg.to_string(),
        error_name: "Exception".to_string(),
        error_code_number: 0,
        error_origin: None,
        compared_values: None
    }
}

/// Converts a `Vec<u8>` to `u128`.
///
/// # Deprecated
///
/// Use [`crate::encoding::hex_utils::bytes_to_u128`] instead.
///
/// # Panics
///
/// Panics if the vector is not exactly 16 bytes.
#[deprecated(since = "1.0.0", note = "Use crate::encoding::hex_utils::bytes_to_u128 instead")]
pub fn vec_to_u128(vec: &[u8]) -> u128 {
    let mut array = [0u8; 16];
    array.copy_from_slice(vec);
    u128::from_be_bytes(array)
}

/// Converts bytes to binary representation.
///
/// # Deprecated
///
/// This function is deprecated and will be removed in a future version.
#[deprecated(since = "1.0.0", note = "Will be removed in 2.0")]
pub fn bytes_to_binary(i: &[u8; 32], r: &mut Vec<u8>) {
    for m in i.iter() {
        format!("{:8b}", m).chars().for_each(|b| if b == '1' { r.push(1); } else { r.push(0) } );
    }
}

// Conversion from legacy error to new error
impl From<SolanaError> for crate::Error {
    fn from(err: SolanaError) -> Self {
        crate::Error::ParseError(err.error_msg)
    }
}