//! Hexadecimal encoding and decoding utilities.
//!
//! This module provides convenient functions for working with hexadecimal
//! representations of binary data.

use crate::error::{Error, Result};

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// Encodes bytes as a hexadecimal string.
///
/// # Arguments
///
/// * `bytes` - The bytes to encode
///
/// # Returns
///
/// A lowercase hexadecimal string representation.
///
/// # Example
///
/// ```
/// use stealth_lib::encoding::encode_hex;
///
/// let hex = encode_hex(&[0xde, 0xad, 0xbe, 0xef]);
/// assert_eq!(hex, "deadbeef");
/// ```
pub fn encode_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Decodes a hexadecimal string to bytes.
///
/// # Arguments
///
/// * `hex_str` - The hexadecimal string to decode (with or without "0x" prefix)
///
/// # Returns
///
/// The decoded bytes, or an error if the input is invalid.
///
/// # Errors
///
/// Returns [`Error::ParseError`] if the string contains invalid hex characters
/// or has an odd length.
///
/// # Example
///
/// ```
/// use stealth_lib::encoding::decode_hex;
///
/// let bytes = decode_hex("deadbeef").unwrap();
/// assert_eq!(bytes, vec![0xde, 0xad, 0xbe, 0xef]);
///
/// // Also works with 0x prefix
/// let bytes = decode_hex("0xdeadbeef").unwrap();
/// assert_eq!(bytes, vec![0xde, 0xad, 0xbe, 0xef]);
/// ```
pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>> {
    let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    hex::decode(hex_str).map_err(|e| Error::ParseError(e.to_string()))
}

/// Converts bytes to a u128 value.
///
/// # Arguments
///
/// * `bytes` - Exactly 16 bytes to convert
///
/// # Returns
///
/// The u128 value, or an error if the input length is wrong.
///
/// # Errors
///
/// Returns [`Error::InvalidLength`] if bytes is not exactly 16 bytes.
///
/// # Example
///
/// ```
/// use stealth_lib::encoding::hex_utils::bytes_to_u128;
///
/// let bytes = [0u8; 16];
/// let value = bytes_to_u128(&bytes).unwrap();
/// assert_eq!(value, 0);
/// ```
pub fn bytes_to_u128(bytes: &[u8]) -> Result<u128> {
    if bytes.len() != 16 {
        return Err(Error::InvalidLength {
            expected: 16,
            actual: bytes.len(),
        });
    }

    let mut array = [0u8; 16];
    array.copy_from_slice(bytes);
    Ok(u128::from_be_bytes(array))
}

/// Converts a u128 value to bytes.
///
/// # Arguments
///
/// * `value` - The u128 value to convert
///
/// # Returns
///
/// A 16-byte array in big-endian order.
///
/// # Example
///
/// ```
/// use stealth_lib::encoding::hex_utils::u128_to_bytes;
///
/// let bytes = u128_to_bytes(0x0102030405060708090a0b0c0d0e0f10);
/// assert_eq!(bytes[0], 0x01);
/// assert_eq!(bytes[15], 0x10);
/// ```
pub fn u128_to_bytes(value: u128) -> [u8; 16] {
    value.to_be_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_hex() {
        assert_eq!(encode_hex(&[]), "");
        assert_eq!(encode_hex(&[0x00]), "00");
        assert_eq!(encode_hex(&[0xff]), "ff");
        assert_eq!(encode_hex(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
    }

    #[test]
    fn test_decode_hex() {
        assert_eq!(decode_hex("").unwrap(), vec![]);
        assert_eq!(decode_hex("00").unwrap(), vec![0x00]);
        assert_eq!(decode_hex("ff").unwrap(), vec![0xff]);
        assert_eq!(decode_hex("deadbeef").unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_decode_hex_with_prefix() {
        assert_eq!(decode_hex("0x").unwrap(), vec![]);
        assert_eq!(decode_hex("0xdeadbeef").unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_decode_hex_invalid() {
        assert!(decode_hex("gg").is_err());
        assert!(decode_hex("0").is_err()); // Odd length
    }

    #[test]
    fn test_bytes_to_u128() {
        let bytes = [0u8; 16];
        assert_eq!(bytes_to_u128(&bytes).unwrap(), 0);

        let bytes = [0xff; 16];
        assert_eq!(bytes_to_u128(&bytes).unwrap(), u128::MAX);
    }

    #[test]
    fn test_bytes_to_u128_wrong_length() {
        assert!(bytes_to_u128(&[0u8; 15]).is_err());
        assert!(bytes_to_u128(&[0u8; 17]).is_err());
    }

    #[test]
    fn test_u128_to_bytes_roundtrip() {
        for value in [0u128, 1, u128::MAX, 12345678901234567890] {
            let bytes = u128_to_bytes(value);
            let recovered = bytes_to_u128(&bytes).unwrap();
            assert_eq!(value, recovered);
        }
    }
}
