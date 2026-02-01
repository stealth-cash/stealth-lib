//! Merkle tree implementation with MiMC hash.
//!
//! This module provides a Merkle tree data structure optimized for use in
//! zero-knowledge proof systems like Tornado Cash and Semaphore.
//!
//! # Features
//!
//! - MiMC-based hashing for ZK-circuit compatibility
//! - Root history buffer for handling concurrent insertions
//! - Proof generation and verification
//! - Serialization support (borsh, serde)
//!
//! # Example
//!
//! ```
//! use stealth_lib::merkle::{MerkleTree, MerkleProof};
//!
//! // Create a tree with 20 levels (can hold 2^20 = ~1M leaves)
//! let mut tree = MerkleTree::new(20).unwrap();
//!
//! // Insert some leaves
//! let idx0 = tree.insert(12345).unwrap();
//! let idx1 = tree.insert(67890).unwrap();
//!
//! // Generate a proof for the first leaf
//! let proof = tree.prove(idx0).unwrap();
//!
//! // Verify the proof
//! let root = tree.root().unwrap();
//! assert!(proof.verify(root, &tree.hasher()));
//! ```
//!
//! # Security Considerations
//!
//! - The tree uses MiMC hashing which is NOT constant-time
//! - Root history prevents front-running in on-chain applications
//! - Proofs should be verified against known roots only

pub mod proof;
pub mod tree;

pub use proof::MerkleProof;
pub use tree::MerkleTree;

/// Default root history size.
///
/// The tree maintains a circular buffer of recent roots to handle
/// concurrent insertions in on-chain applications.
pub const ROOT_HISTORY_SIZE: u8 = 30;
