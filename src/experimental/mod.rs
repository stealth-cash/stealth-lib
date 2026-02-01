//! ⚠️ EXPERIMENTAL: Educational implementations only.
//!
//! # WARNING: DO NOT USE IN PRODUCTION
//!
//! The implementations in this module are provided for **educational purposes only**.
//! They have **NOT been audited**, may contain **security vulnerabilities**, and are
//! **NOT suitable for any production use**.
//!
//! This module is gated behind the `experimental` feature flag and will emit
//! deprecation warnings when used.
//!
//! # What's Here
//!
//! - [`sha1`] - A from-scratch SHA-1 implementation (SHA-1 is cryptographically broken!)
//! - [`prng`] - A simple PRNG (NOT cryptographically secure!)
//!
//! # What You Should Use Instead
//!
//! | This Module | Production Alternative |
//! |-------------|----------------------|
//! | `sha1` | [`sha2`](https://crates.io/crates/sha2) for SHA-256/512 |
//! | `prng` | [`rand`](https://crates.io/crates/rand) + [`getrandom`](https://crates.io/crates/getrandom) |
//!
//! # Example (Don't do this in production!)
//!
//! ```ignore
//! // This code requires the `experimental` feature
//! use stealth_lib::experimental::sha1::Sha1;
//!
//! // WARNING: SHA-1 is broken! This is for learning only!
//! let hash = Sha1::hash("hello world");
//! ```

#[cfg(feature = "experimental")]
pub mod sha1;

#[cfg(feature = "experimental")]
pub mod prng;

// Compile-time warning when experimental feature is enabled
// Note: This produces a deprecation warning intentionally to alert users
#[cfg(feature = "experimental")]
#[allow(dead_code)]
#[deprecated(
    since = "1.0.0",
    note = "The 'experimental' feature enables insecure educational code. DO NOT USE IN PRODUCTION."
)]
const EXPERIMENTAL_WARNING: &str = "UNSAFE";
