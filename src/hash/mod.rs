//! Hash functions for stealth-lib.
//!
//! This module provides ZK-friendly hash functions designed for use in
//! zero-knowledge proof circuits.
//!
//! # Available Hash Functions
//!
//! - [`MimcHasher`] - MiMC-Feistel sponge construction
//!
//! # Security Note
//!
//! These hash functions are designed for ZK circuits (e.g., Tornado Cash, Semaphore)
//! and are **NOT** general-purpose cryptographic hash functions. They are:
//!
//! - **NOT constant-time** (vulnerable to timing side-channels)
//! - **NOT suitable for password hashing**
//! - **NOT a replacement for SHA-256, BLAKE3, etc.**
//!
//! For general-purpose hashing, use established crates like `sha2`, `blake3`, or `ring`.

pub mod mimc;

pub use mimc::MimcHasher;
