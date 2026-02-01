//! Error types for stealth-lib operations.
//!
//! This module provides a unified error type for all operations in the library.
//! All errors are typed and provide meaningful context for debugging.

use core::fmt;

/// All errors that can occur in stealth-lib operations.
///
/// This enum is `#[non_exhaustive]` to allow adding new variants in future
/// minor versions without breaking semver compatibility.
///
/// # Example
///
/// ```
/// use stealth_lib::Error;
///
/// fn example() -> Result<(), Error> {
///     Err(Error::TreeFull {
///         capacity: 1048576,
///         attempted_index: 1048576,
///     })
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Merkle tree has reached maximum capacity.
    ///
    /// This occurs when attempting to insert a leaf into a full tree.
    /// The tree capacity is `2^levels`.
    TreeFull {
        /// Maximum number of leaves the tree can hold.
        capacity: usize,
        /// The index that was attempted.
        attempted_index: usize,
    },

    /// Invalid Merkle proof.
    ///
    /// The proof does not verify against the expected root.
    InvalidProof,

    /// Root not found in history.
    ///
    /// The provided root hash is not in the tree's root history buffer.
    UnknownRoot,

    /// Input data has invalid length.
    ///
    /// Expected a specific number of bytes but received a different amount.
    InvalidLength {
        /// Expected length in bytes.
        expected: usize,
        /// Actual length received.
        actual: usize,
    },

    /// Parsing failed.
    ///
    /// Failed to parse input data (e.g., from string representation).
    ParseError(String),

    /// Arithmetic overflow in field operations.
    ///
    /// An arithmetic operation would overflow the field modulus.
    FieldOverflow,

    /// Invalid tree configuration.
    ///
    /// The tree parameters are invalid (e.g., zero levels).
    InvalidTreeConfig(String),

    /// Leaf index out of bounds.
    ///
    /// The requested leaf index does not exist in the tree.
    LeafIndexOutOfBounds {
        /// The requested index.
        index: u32,
        /// The current number of leaves in the tree.
        tree_size: u32,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TreeFull {
                capacity,
                attempted_index,
            } => {
                write!(
                    f,
                    "Merkle tree is full: capacity={}, attempted index={}",
                    capacity, attempted_index
                )
            }
            Error::InvalidProof => write!(f, "Invalid Merkle proof"),
            Error::UnknownRoot => write!(f, "Root not found in history"),
            Error::InvalidLength { expected, actual } => {
                write!(
                    f,
                    "Invalid length: expected {} bytes, got {}",
                    expected, actual
                )
            }
            Error::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Error::FieldOverflow => write!(f, "Arithmetic overflow in field operation"),
            Error::InvalidTreeConfig(msg) => write!(f, "Invalid tree configuration: {}", msg),
            Error::LeafIndexOutOfBounds { index, tree_size } => {
                write!(
                    f,
                    "Leaf index {} out of bounds (tree has {} leaves)",
                    index, tree_size
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Result type alias for stealth-lib operations.
///
/// This is a convenience alias that uses [`Error`] as the error type.
///
/// # Example
///
/// ```
/// use stealth_lib::{Result, MerkleTree};
///
/// fn insert_leaf(tree: &mut MerkleTree, leaf: u128) -> Result<u32> {
///     tree.insert(leaf)
/// }
/// ```
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::TreeFull {
            capacity: 100,
            attempted_index: 100,
        };
        assert!(err.to_string().contains("100"));

        let err = Error::ParseError("invalid input".to_string());
        assert!(err.to_string().contains("invalid input"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = Error::InvalidProof;
        let err2 = Error::InvalidProof;
        assert_eq!(err1, err2);

        let err3 = Error::UnknownRoot;
        assert_ne!(err1, err3);
    }
}
