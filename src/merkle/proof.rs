//! Merkle proof generation and verification.
//!
//! This module provides the [`MerkleProof`] type for proving membership
//! of a leaf in a Merkle tree.

use crate::hash::MimcHasher;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// A Merkle inclusion proof.
///
/// This proof demonstrates that a specific leaf is included in a Merkle tree
/// at a specific position. The proof consists of the sibling hashes along
/// the path from the leaf to the root.
///
/// # Structure
///
/// - `leaf`: The leaf value being proven
/// - `leaf_index`: Position of the leaf in the tree (0-indexed)
/// - `path`: Sibling hashes from leaf to root
/// - `indices`: Direction at each level (false=left, true=right)
///
/// # Example
///
/// ```
/// use stealth_lib::{MerkleTree, MerkleProof};
///
/// let mut tree = MerkleTree::new(10).unwrap();
/// tree.insert(12345).unwrap();
///
/// let proof = tree.prove(0).unwrap();
/// let root = tree.root().unwrap();
///
/// assert!(proof.verify(root, &tree.hasher()));
/// ```
///
/// # Security Note
///
/// Always verify proofs against a known, trusted root hash. An attacker
/// can construct valid proofs for any leaf given any root if they control
/// the proof data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleProof {
    /// The leaf value being proven.
    pub leaf: u128,
    /// Index of the leaf in the tree (0-indexed).
    pub leaf_index: u32,
    /// Sibling hashes along the path from leaf to root.
    pub path: Vec<u128>,
    /// Direction indicators at each level.
    /// `false` = leaf is on the left, `true` = leaf is on the right.
    pub indices: Vec<bool>,
}

impl MerkleProof {
    /// Creates a new Merkle proof.
    ///
    /// # Arguments
    ///
    /// * `leaf` - The leaf value
    /// * `leaf_index` - Index of the leaf in the tree
    /// * `path` - Sibling hashes from leaf to root
    /// * `indices` - Direction indicators at each level
    pub fn new(leaf: u128, leaf_index: u32, path: Vec<u128>, indices: Vec<bool>) -> Self {
        MerkleProof {
            leaf,
            leaf_index,
            path,
            indices,
        }
    }

    /// Returns the depth of this proof (number of levels).
    #[inline]
    pub fn depth(&self) -> usize {
        self.path.len()
    }

    /// Verifies this proof against a root hash.
    ///
    /// # Arguments
    ///
    /// * `root` - The expected root hash
    /// * `hasher` - The MiMC hasher used by the tree
    ///
    /// # Returns
    ///
    /// `true` if the proof is valid for the given root, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::{MerkleTree, MerkleProof};
    ///
    /// let mut tree = MerkleTree::new(10).unwrap();
    /// tree.insert(12345).unwrap();
    ///
    /// let proof = tree.prove(0).unwrap();
    /// let root = tree.root().unwrap();
    ///
    /// // Valid proof
    /// assert!(proof.verify(root, &tree.hasher()));
    ///
    /// // Invalid root
    /// assert!(!proof.verify(99999, &tree.hasher()));
    /// ```
    pub fn verify(&self, root: u128, hasher: &MimcHasher) -> bool {
        if self.path.len() != self.indices.len() {
            return false;
        }

        let computed_root = self.compute_root(hasher);
        computed_root == root
    }

    /// Computes the root hash from this proof.
    ///
    /// This walks up the tree from the leaf, combining with siblings
    /// according to the direction indicators.
    ///
    /// # Arguments
    ///
    /// * `hasher` - The MiMC hasher used by the tree
    ///
    /// # Returns
    ///
    /// The computed root hash.
    pub fn compute_root(&self, hasher: &MimcHasher) -> u128 {
        let field_size = hasher.field_prime();
        let c = 0_u128;

        let mut current = self.leaf;

        for (sibling, &is_right) in self.path.iter().zip(self.indices.iter()) {
            let (left, right) = if is_right {
                (*sibling, current)
            } else {
                (current, *sibling)
            };

            // Hash left and right children (same algorithm as tree)
            let mut r = left;
            r = hasher.mimc_sponge(r, c, field_size);
            r = r.wrapping_add(right).wrapping_rem(field_size);
            r = hasher.mimc_sponge(r, c, field_size);
            current = r;
        }

        current
    }

    /// Returns the leaf value.
    #[inline]
    pub fn leaf(&self) -> u128 {
        self.leaf
    }

    /// Returns the leaf index.
    #[inline]
    pub fn leaf_index(&self) -> u32 {
        self.leaf_index
    }

    /// Returns the path (sibling hashes).
    #[inline]
    pub fn path(&self) -> &[u128] {
        &self.path
    }

    /// Returns the direction indices.
    #[inline]
    pub fn indices(&self) -> &[bool] {
        &self.indices
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::Serialize;

    impl Serialize for MerkleProof {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut state = serializer.serialize_struct("MerkleProof", 4)?;
            state.serialize_field("leaf", &self.leaf)?;
            state.serialize_field("leaf_index", &self.leaf_index)?;
            state.serialize_field("path", &self.path)?;
            state.serialize_field("indices", &self.indices)?;
            state.end()
        }
    }
}

#[cfg(feature = "borsh")]
mod borsh_impl {
    use super::*;
    use borsh::{BorshDeserialize, BorshSerialize};

    impl BorshSerialize for MerkleProof {
        fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
            self.leaf.serialize(writer)?;
            self.leaf_index.serialize(writer)?;
            self.path.serialize(writer)?;
            self.indices.serialize(writer)?;
            Ok(())
        }
    }

    impl BorshDeserialize for MerkleProof {
        fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
            let leaf = u128::deserialize_reader(reader)?;
            let leaf_index = u32::deserialize_reader(reader)?;
            let path = Vec::<u128>::deserialize_reader(reader)?;
            let indices = Vec::<bool>::deserialize_reader(reader)?;
            Ok(MerkleProof {
                leaf,
                leaf_index,
                path,
                indices,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_hasher() -> MimcHasher {
        MimcHasher::default()
    }

    #[test]
    fn test_proof_new() {
        let proof = MerkleProof::new(12345, 0, vec![1, 2, 3], vec![false, true, false]);
        assert_eq!(proof.leaf(), 12345);
        assert_eq!(proof.leaf_index(), 0);
        assert_eq!(proof.depth(), 3);
    }

    #[test]
    fn test_proof_depth() {
        let proof = MerkleProof::new(0, 0, vec![1, 2, 3, 4, 5], vec![false; 5]);
        assert_eq!(proof.depth(), 5);
    }

    #[test]
    fn test_proof_mismatched_lengths_fails_verify() {
        let proof = MerkleProof {
            leaf: 12345,
            leaf_index: 0,
            path: vec![1, 2, 3],
            indices: vec![false, true], // Wrong length!
        };
        assert!(!proof.verify(0, &default_hasher()));
    }

    #[test]
    fn test_compute_root_deterministic() {
        let proof = MerkleProof::new(12345, 0, vec![1, 2, 3], vec![false, false, false]);
        let hasher = default_hasher();

        let root1 = proof.compute_root(&hasher);
        let root2 = proof.compute_root(&hasher);
        assert_eq!(root1, root2);
    }

    #[test]
    fn test_verify_wrong_root_fails() {
        let proof = MerkleProof::new(12345, 0, vec![1, 2, 3], vec![false, false, false]);
        let hasher = default_hasher();

        let computed = proof.compute_root(&hasher);
        assert!(proof.verify(computed, &hasher));
        assert!(!proof.verify(computed + 1, &hasher));
    }
}
