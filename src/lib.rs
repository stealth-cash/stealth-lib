//! # stealth-lib
//!
//! ZK-friendly cryptographic primitives for Rust.
//!
//! This library provides cryptographic primitives designed for use in zero-knowledge
//! proof systems like Tornado Cash, Semaphore, and similar applications.
//!
//! ## Features
//!
//! - **MiMC Hash**: Efficient hash function designed for ZK circuits
//! - **Merkle Tree**: MiMC-based tree with proof generation and verification
//! - **No unsafe code**: `#![deny(unsafe_code)]`
//! - **`no_std` support**: Optional, for WASM/embedded targets
//!
//! ## Quick Start
//!
//! ```
//! use stealth_lib::{MerkleTree, MerkleProof};
//!
//! // Create a Merkle tree with 20 levels (can hold ~1M leaves)
//! let mut tree = MerkleTree::new(20).unwrap();
//!
//! // Insert some leaves
//! let idx = tree.insert(12345).unwrap();
//!
//! // Generate and verify a proof
//! let proof = tree.prove(idx).unwrap();
//! let root = tree.root().unwrap();
//! assert!(proof.verify(root, &tree.hasher()));
//! ```
//!
//! ## Security Model
//!
//! **Designed for**: Zero-knowledge proof circuits (Tornado Cash, Semaphore, etc.)
//!
//! **Guarantees**:
//! - Collision resistance of MiMC (computational)
//! - Correct Merkle proofs for membership verification
//!
//! **Non-Goals / Explicit Exclusions**:
//! - ❌ Constant-time execution (vulnerable to timing side-channels)
//! - ❌ General-purpose cryptographic primitives
//! - ❌ Professional security audit (pending)
//!
//! For general-purpose cryptography, use established crates like `ring`, `sha2`,
//! `ed25519-dalek`, etc.
//!
//! ## Feature Flags
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `std` | ✅ | Enable standard library support |
//! | `serde` | ❌ | Enable serde serialization |
//! | `borsh` | ❌ | Enable borsh serialization |
//! | `experimental` | ❌ | ⚠️ Educational code only, NOT for production |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]

#[cfg(not(feature = "std"))]
extern crate alloc;

// Core modules
pub mod error;
pub mod hash;
pub mod merkle;
pub mod encoding;

// Experimental/educational modules (feature-gated)
#[cfg(feature = "experimental")]
pub mod experimental;

// Legacy modules (deprecated, will be removed in 2.0)
#[deprecated(since = "1.0.0", note = "Use hash::mimc::MimcHasher instead")]
pub mod hasher;
#[deprecated(since = "1.0.0", note = "Use merkle::MerkleTree instead")]
pub mod merkle_tree;
#[deprecated(since = "1.0.0", note = "Use error module instead")]
pub mod utils;

// Public API re-exports
pub use error::{Error, Result};
pub use hash::MimcHasher;
pub use merkle::{MerkleProof, MerkleTree};

// Backwards compatibility type alias
#[deprecated(since = "1.0.0", note = "Renamed to Error")]
#[allow(missing_docs)]
pub type SolanaError = Error;