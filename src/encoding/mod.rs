//! Encoding utilities.
//!
//! This module provides utilities for encoding and decoding data,
//! primarily wrapping the `hex` crate for hexadecimal operations.

pub mod hex_utils;

pub use hex_utils::{decode_hex, encode_hex};
