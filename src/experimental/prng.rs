//! ⚠️ EDUCATIONAL ONLY: Pseudo-random number generator.
//!
//! # WARNING
//!
//! This implementation is provided for **educational purposes only**.
//!
//! **DO NOT USE THIS IN PRODUCTION** because:
//!
//! 1. **NOT cryptographically secure** - Trivially predictable output
//! 2. **Weak entropy source** - Uses predictable system values
//! 3. **Platform-specific** - May panic on some platforms
//! 4. **Not audited** - May contain bugs
//!
//! # What to Use Instead
//!
//! For production use, use:
//!
//! - [`getrandom`](https://crates.io/crates/getrandom) - OS-level CSPRNG
//! - [`rand`](https://crates.io/crates/rand) - High-level random number generation
//! - [`rand_chacha`](https://crates.io/crates/rand_chacha) - ChaCha-based CSPRNG
//!
//! ```ignore
//! use rand::Rng;
//!
//! let mut rng = rand::thread_rng();
//! let random_number: u64 = rng.gen();
//! ```
//!
//! # Educational Purpose
//!
//! This code demonstrates concepts of random number generation:
//! - Entropy collection from system sources
//! - PRNG state management
//! - The importance of proper seeding

#![allow(dead_code)]

use core::hash::{Hash, Hasher};

#[cfg(feature = "std")]
use std::collections::hash_map::DefaultHasher;
#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

/// A simple PRNG (EDUCATIONAL ONLY).
///
/// # Security Warning
///
/// This is **NOT cryptographically secure**. Do not use for:
/// - Key generation
/// - Nonces/IVs
/// - Session tokens
/// - Any security-critical application
#[deprecated(
    since = "1.0.0",
    note = "Not cryptographically secure. Use rand + getrandom for production."
)]
pub struct SimplePrng {
    state: u64,
}

#[allow(deprecated)]
impl SimplePrng {
    /// Creates a new PRNG with a seed derived from system time.
    ///
    /// # Warning
    ///
    /// The seed is derived from system time, which is predictable.
    /// Do not use for security-sensitive applications.
    #[cfg(feature = "std")]
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        Self::with_seed(seed)
    }

    /// Creates a new PRNG with the specified seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value
    pub fn with_seed(seed: u64) -> Self {
        SimplePrng {
            state: hash_value(&seed),
        }
    }

    /// Generates the next random u64 value.
    ///
    /// Uses a simple xorshift algorithm (NOT secure!).
    pub fn next_u64(&mut self) -> u64 {
        // xorshift64
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Generates a random value in the range [min, max).
    ///
    /// # Arguments
    ///
    /// * `min` - Inclusive lower bound
    /// * `max` - Exclusive upper bound
    ///
    /// # Returns
    ///
    /// A random value, or `None` if min >= max.
    pub fn range(&mut self, min: u64, max: u64) -> Option<u64> {
        if min >= max {
            return None;
        }
        let range = max - min;
        Some((self.next_u64() % range) + min)
    }

    /// Generates a random boolean.
    pub fn next_bool(&mut self) -> bool {
        self.next_u64() % 2 == 0
    }
}

#[cfg(feature = "std")]
#[allow(deprecated)]
impl Default for SimplePrng {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple hash function for seeding (NOT cryptographic).
#[cfg(feature = "std")]
fn hash_value<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(not(feature = "std"))]
fn hash_value<T>(value: &T) -> u64 {
    // Fallback for no_std - just use a constant
    // This is intentionally bad for educational purposes
    0xdeadbeef
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_prng_deterministic_with_seed() {
        let mut prng1 = SimplePrng::with_seed(12345);
        let mut prng2 = SimplePrng::with_seed(12345);

        for _ in 0..100 {
            assert_eq!(prng1.next_u64(), prng2.next_u64());
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_prng_different_seeds() {
        let mut prng1 = SimplePrng::with_seed(12345);
        let mut prng2 = SimplePrng::with_seed(54321);

        // Very likely to be different
        assert_ne!(prng1.next_u64(), prng2.next_u64());
    }

    #[test]
    #[allow(deprecated)]
    fn test_prng_range() {
        let mut prng = SimplePrng::with_seed(12345);

        for _ in 0..1000 {
            let value = prng.range(10, 20).unwrap();
            assert!(value >= 10 && value < 20);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_prng_range_invalid() {
        let mut prng = SimplePrng::with_seed(12345);
        assert!(prng.range(20, 10).is_none());
        assert!(prng.range(10, 10).is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn test_prng_bool() {
        let mut prng = SimplePrng::with_seed(12345);
        let mut trues = 0;
        let mut falses = 0;

        for _ in 0..1000 {
            if prng.next_bool() {
                trues += 1;
            } else {
                falses += 1;
            }
        }

        // Should have a roughly even distribution
        assert!(trues > 300 && trues < 700);
        assert!(falses > 300 && falses < 700);
    }
}
