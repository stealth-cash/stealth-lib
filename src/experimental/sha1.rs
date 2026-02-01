//! ⚠️ EDUCATIONAL ONLY: SHA-1 hash function implementation.
//!
//! # WARNING
//!
//! This implementation is provided for **educational purposes only**.
//!
//! **DO NOT USE THIS IN PRODUCTION** because:
//!
//! 1. **SHA-1 is cryptographically broken** - Collision attacks are practical
//! 2. **This implementation is not audited** - It may contain bugs
//! 3. **Not constant-time** - Vulnerable to timing attacks
//! 4. **No test vectors** - Not verified against NIST test vectors
//!
//! # What to Use Instead
//!
//! For production use, use the [`sha2`](https://crates.io/crates/sha2) crate:
//!
//! ```ignore
//! use sha2::{Sha256, Digest};
//!
//! let mut hasher = Sha256::new();
//! hasher.update(b"hello world");
//! let result = hasher.finalize();
//! ```
//!
//! # Educational Purpose
//!
//! This code demonstrates the structure of the SHA-1 algorithm:
//! - Message padding
//! - Block processing
//! - Merkle-Damgård construction
//! - Bitwise operations in hash functions

#![allow(dead_code)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec, vec::Vec};

/// SHA-1 hash function (EDUCATIONAL ONLY).
///
/// # Security Warning
///
/// SHA-1 is **cryptographically broken**. Do not use for:
/// - Digital signatures
/// - Certificate validation  
/// - Password hashing
/// - Any security-critical application
///
/// Use SHA-256 or SHA-3 instead.
#[deprecated(
    since = "1.0.0",
    note = "SHA-1 is cryptographically broken. Use sha2 crate for production."
)]
pub struct Sha1;

#[allow(deprecated)]
impl Sha1 {
    /// Computes the SHA-1 hash of the input string.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to hash
    ///
    /// # Returns
    ///
    /// The hash as a 40-character hexadecimal string.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use stealth_lib::experimental::sha1::Sha1;
    ///
    /// let hash = Sha1::hash("hello");
    /// // Note: This is educational code, don't use in production!
    /// ```
    pub fn hash(input: &str) -> String {
        let payload = input.as_bytes();

        // Initial hash values (first 32 bits of fractional parts of square roots of first 5 primes)
        let mut h0 = 0x67452301u32;
        let mut h1 = 0xEFCDAB89u32;
        let mut h2 = 0x98BADCFEu32;
        let mut h3 = 0x10325476u32;
        let mut h4 = 0xC3D2E1F0u32;

        // Original message length in bits
        let ml = (payload.len() as u64) * 8;

        // Padding
        let mut message = payload.to_vec();
        message.push(0x80);

        while (message.len() * 8) % 512 != 448 {
            message.push(0);
        }

        // Append length as 64-bit big-endian
        message.extend_from_slice(&ml.to_be_bytes());

        // Process each 512-bit block
        for chunk in message.chunks(64) {
            // Prepare message schedule
            let mut words = vec![0u32; 80];

            #[allow(clippy::needless_range_loop)]
            for i in 0..16 {
                let start = i * 4;
                words[i] = u32::from_be_bytes([
                    chunk[start],
                    chunk[start + 1],
                    chunk[start + 2],
                    chunk[start + 3],
                ]);
            }

            // Extend the sixteen 32-bit words into eighty 32-bit words
            for i in 16..80 {
                words[i] = (words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16])
                    .rotate_left(1);
            }

            // Initialize working variables
            let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

            // Main loop
            #[allow(clippy::needless_range_loop)]
            for i in 0..80 {
                let (f, k) = match i {
                    0..=19 => ((b & c) | ((!b) & d), 0x5A827999u32),
                    20..=39 => (b ^ c ^ d, 0x6ED9EBA1u32),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDCu32),
                    60..=79 => (b ^ c ^ d, 0xCA62C1D6u32),
                    _ => unreachable!(),
                };

                let temp = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(words[i]);

                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            // Add this chunk's hash to result
            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }

        // Produce the final hash value (160 bits / 20 bytes)
        format!(
            "{:08x}{:08x}{:08x}{:08x}{:08x}",
            h0, h1, h2, h3, h4
        )
    }

    /// Verifies that a string hashes to the expected value.
    ///
    /// # Arguments
    ///
    /// * `input` - The original string
    /// * `expected_hash` - The expected SHA-1 hash (hex string)
    ///
    /// # Returns
    ///
    /// `true` if the hash matches, `false` otherwise.
    pub fn verify(input: &str, expected_hash: &str) -> bool {
        Self::hash(input) == expected_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_sha1_empty() {
        // SHA-1("") = da39a3ee5e6b4b0d3255bfef95601890afd80709
        let hash = Sha1::hash("");
        assert_eq!(hash, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    #[allow(deprecated)]
    fn test_sha1_hello() {
        // SHA-1("hello") = aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d
        let hash = Sha1::hash("hello");
        assert_eq!(hash, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }

    #[test]
    #[allow(deprecated)]
    fn test_sha1_abc() {
        // SHA-1("abc") = a9993e364706816aba3e25717850c26c9cd0d89d
        let hash = Sha1::hash("abc");
        assert_eq!(hash, "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    #[allow(deprecated)]
    fn test_sha1_verify() {
        assert!(Sha1::verify("hello", "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"));
        assert!(!Sha1::verify("hello", "wrong_hash"));
    }
}
