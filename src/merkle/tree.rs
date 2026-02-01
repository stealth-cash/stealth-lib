//! Merkle tree data structure.
//!
//! A sparse Merkle tree implementation using MiMC hash, designed for
//! zero-knowledge proof applications.

use crate::error::{Error, Result};
use crate::hash::MimcHasher;
use crate::merkle::proof::MerkleProof;
use crate::merkle::ROOT_HISTORY_SIZE;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// A Merkle tree with MiMC hash function.
///
/// This implementation is optimized for ZK-circuit compatibility and includes
/// features like root history for handling concurrent on-chain insertions.
///
/// # Example
///
/// ```
/// use stealth_lib::MerkleTree;
///
/// // Create a new tree with 20 levels
/// let mut tree = MerkleTree::new(20).unwrap();
///
/// // Insert leaves
/// let index = tree.insert(12345).unwrap();
/// assert_eq!(index, 0);
///
/// // Get the current root
/// let root = tree.root().unwrap();
/// println!("Root: {}", root);
/// ```
///
/// # Capacity
///
/// A tree with `n` levels can hold `2^n` leaves. The maximum supported
/// depth is 255 levels, though practical trees typically use 20-32 levels.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Number of levels in the tree (excluding root).
    levels: u8,
    /// Pre-computed subtree hashes for empty positions.
    filled_subtrees: HashMap<u8, u128>,
    /// Circular buffer of recent root hashes.
    roots: HashMap<u8, u128>,
    /// Index into the roots circular buffer.
    current_root_index: u8,
    /// Index for the next leaf to be inserted.
    next_index: u32,
    /// Hash function used for the tree.
    hasher: MimcHasher,
    /// Leaves inserted into the tree (for proof generation).
    leaves: Vec<u128>,
}

impl MerkleTree {
    /// Creates a new empty Merkle tree with the specified number of levels.
    ///
    /// # Arguments
    ///
    /// * `levels` - The depth of the tree. The tree can hold `2^levels` leaves.
    ///
    /// # Returns
    ///
    /// A new `MerkleTree` or an error if the configuration is invalid.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTreeConfig`] if `levels` is 0 or greater than 32.
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::MerkleTree;
    ///
    /// let tree = MerkleTree::new(20).unwrap();
    /// assert_eq!(tree.levels(), 20);
    /// assert_eq!(tree.capacity(), 1 << 20);
    /// ```
    pub fn new(levels: u8) -> Result<Self> {
        if levels == 0 {
            return Err(Error::InvalidTreeConfig(
                "Tree must have at least 1 level".to_string(),
            ));
        }
        if levels > 32 {
            return Err(Error::InvalidTreeConfig(
                "Tree depth cannot exceed 32 levels".to_string(),
            ));
        }

        let hasher = MimcHasher::default();
        let mut instance = MerkleTree {
            levels,
            filled_subtrees: HashMap::new(),
            roots: HashMap::new(),
            current_root_index: 0,
            next_index: 0,
            hasher,
            leaves: Vec::new(),
        };

        // Initialize filled_subtrees with zero hashes
        for i in 0..levels {
            instance.filled_subtrees.insert(i, instance.zeros(i));
        }

        // Initialize root with the empty tree root
        instance.roots.insert(0, instance.zeros(levels - 1));

        Ok(instance)
    }

    /// Creates a new Merkle tree with a custom hasher.
    ///
    /// # Arguments
    ///
    /// * `levels` - The depth of the tree
    /// * `hasher` - Custom MiMC hasher configuration
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::{MerkleTree, hash::MimcHasher};
    ///
    /// let hasher = MimcHasher::default();
    /// let tree = MerkleTree::with_hasher(20, hasher).unwrap();
    /// ```
    pub fn with_hasher(levels: u8, hasher: MimcHasher) -> Result<Self> {
        if levels == 0 {
            return Err(Error::InvalidTreeConfig(
                "Tree must have at least 1 level".to_string(),
            ));
        }
        if levels > 32 {
            return Err(Error::InvalidTreeConfig(
                "Tree depth cannot exceed 32 levels".to_string(),
            ));
        }

        let mut instance = MerkleTree {
            levels,
            filled_subtrees: HashMap::new(),
            roots: HashMap::new(),
            current_root_index: 0,
            next_index: 0,
            hasher,
            leaves: Vec::new(),
        };

        for i in 0..levels {
            instance.filled_subtrees.insert(i, instance.zeros(i));
        }

        instance.roots.insert(0, instance.zeros(levels - 1));

        Ok(instance)
    }

    /// Returns the number of levels in the tree.
    #[inline]
    pub fn levels(&self) -> u8 {
        self.levels
    }

    /// Returns the maximum capacity of the tree.
    ///
    /// This is `2^levels`.
    #[inline]
    pub fn capacity(&self) -> usize {
        1usize << self.levels
    }

    /// Returns the current number of leaves in the tree.
    #[inline]
    pub fn len(&self) -> u32 {
        self.next_index
    }

    /// Returns true if the tree is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.next_index == 0
    }

    /// Returns a reference to the hasher used by this tree.
    #[inline]
    pub fn hasher(&self) -> &MimcHasher {
        &self.hasher
    }

    /// Returns the current root hash of the tree.
    ///
    /// Returns `None` only if the tree is in an invalid state (should not happen
    /// under normal usage).
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::MerkleTree;
    ///
    /// let tree = MerkleTree::new(20).unwrap();
    /// let root = tree.root().unwrap();
    /// println!("Empty tree root: {}", root);
    /// ```
    pub fn root(&self) -> Option<u128> {
        self.roots.get(&self.current_root_index).copied()
    }

    /// Hashes two child nodes to produce a parent node.
    ///
    /// Uses the MiMC sponge construction for ZK-circuit compatibility.
    fn hash_left_right(&self, left: u128, right: u128) -> u128 {
        let field_size = self.hasher.field_prime();
        let c = 0_u128;

        let mut r = left;
        r = self.hasher.mimc_sponge(r, c, field_size);
        r = r.wrapping_add(right).wrapping_rem(field_size);
        r = self.hasher.mimc_sponge(r, c, field_size);

        r
    }

    /// Inserts a new leaf into the tree.
    ///
    /// # Arguments
    ///
    /// * `leaf` - The leaf value to insert
    ///
    /// # Returns
    ///
    /// The index of the inserted leaf, or an error if the tree is full.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TreeFull`] if the tree has reached its maximum capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::MerkleTree;
    ///
    /// let mut tree = MerkleTree::new(20).unwrap();
    /// let index = tree.insert(12345).unwrap();
    /// assert_eq!(index, 0);
    ///
    /// let index = tree.insert(67890).unwrap();
    /// assert_eq!(index, 1);
    /// ```
    pub fn insert(&mut self, leaf: u128) -> Result<u32> {
        let capacity = self.capacity();
        if (self.next_index as usize) >= capacity {
            return Err(Error::TreeFull {
                capacity,
                attempted_index: self.next_index as usize,
            });
        }

        let inserted_index = self.next_index;
        let mut current_index = self.next_index;
        let mut current_level_hash = leaf;

        // Store the leaf for proof generation
        self.leaves.push(leaf);

        // Update the tree path from leaf to root
        for i in 0..self.levels {
            let (left, right) = if current_index % 2 == 0 {
                // This is a left child
                self.filled_subtrees.insert(i, current_level_hash);
                (current_level_hash, self.zeros(i))
            } else {
                // This is a right child
                let left = self
                    .filled_subtrees
                    .get(&i)
                    .copied()
                    .unwrap_or_else(|| self.zeros(i));
                (left, current_level_hash)
            };

            current_level_hash = self.hash_left_right(left, right);
            current_index /= 2;
        }

        // Update root history
        let new_root_index = (self.current_root_index + 1) % ROOT_HISTORY_SIZE;
        self.current_root_index = new_root_index;
        self.roots.insert(new_root_index, current_level_hash);
        self.next_index = inserted_index + 1;

        Ok(inserted_index)
    }

    /// Checks if a root hash is in the recent root history.
    ///
    /// The tree maintains a circular buffer of recent roots to handle
    /// concurrent insertions in on-chain applications.
    ///
    /// # Arguments
    ///
    /// * `root` - The root hash to check
    ///
    /// # Returns
    ///
    /// `true` if the root is in the history, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::MerkleTree;
    ///
    /// let mut tree = MerkleTree::new(20).unwrap();
    /// let root_before = tree.root().unwrap();
    /// tree.insert(12345).unwrap();
    /// let root_after = tree.root().unwrap();
    ///
    /// // Both roots are in history
    /// assert!(tree.is_known_root(root_before));
    /// assert!(tree.is_known_root(root_after));
    ///
    /// // Random value is not
    /// assert!(!tree.is_known_root(99999));
    /// ```
    pub fn is_known_root(&self, root: u128) -> bool {
        if root == 0 {
            return false;
        }

        let mut i = self.current_root_index;
        loop {
            if let Some(&stored_root) = self.roots.get(&i) {
                if stored_root == root {
                    return true;
                }
            }

            i = if i == 0 {
                ROOT_HISTORY_SIZE - 1
            } else {
                i - 1
            };

            if i == self.current_root_index {
                break;
            }
        }

        false
    }

    /// Returns the last (current) root hash.
    ///
    /// # Panics
    ///
    /// Panics if the tree is in an invalid state (should not happen under normal usage).
    /// Prefer using [`root`](Self::root) for fallible access.
    #[deprecated(since = "1.0.0", note = "Use root() instead")]
    pub fn get_last_root(&self) -> u128 {
        self.root().expect("Tree in invalid state: no root")
    }

    /// Computes the zero hash at a given level.
    ///
    /// Zero hashes represent empty subtrees at each level.
    /// This uses the same formula as the original Tornado Cash implementation:
    /// `zeros(0) = 0`, `zeros(i) = mimc_sponge(zeros(i-1), 0, p)`.
    ///
    /// Note: This is NOT the same as `hash_left_right(zeros(i-1), zeros(i-1))`.
    /// The formula is chosen for compatibility with existing ZK circuits.
    pub fn zeros(&self, level: u8) -> u128 {
        let mut result = 0u128;
        for _ in 0..level {
            result = self.hasher.mimc_sponge(result, 0, self.hasher.field_prime());
        }
        result
    }

    /// Generates a Merkle proof for the leaf at the given index.
    ///
    /// # Arguments
    ///
    /// * `leaf_index` - The index of the leaf to prove
    ///
    /// # Returns
    ///
    /// A [`MerkleProof`] that can be used to verify inclusion.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LeafIndexOutOfBounds`] if the index is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::MerkleTree;
    ///
    /// let mut tree = MerkleTree::new(20).unwrap();
    /// tree.insert(12345).unwrap();
    /// tree.insert(67890).unwrap();
    ///
    /// let proof = tree.prove(0).unwrap();
    /// let root = tree.root().unwrap();
    /// assert!(proof.verify(root, &tree.hasher()));
    /// ```
    pub fn prove(&self, leaf_index: u32) -> Result<MerkleProof> {
        if leaf_index >= self.next_index {
            return Err(Error::LeafIndexOutOfBounds {
                index: leaf_index,
                tree_size: self.next_index,
            });
        }

        let leaf = self.leaves[leaf_index as usize];
        let mut path = Vec::with_capacity(self.levels as usize);
        let mut indices = Vec::with_capacity(self.levels as usize);
        let mut current_index = leaf_index;

        for level in 0..self.levels {
            let is_right = current_index % 2 == 1;
            indices.push(is_right);

            // Get sibling
            let sibling_index = if is_right {
                current_index - 1
            } else {
                current_index + 1
            };

            let sibling = self.get_node_at(level, sibling_index);
            path.push(sibling);

            current_index /= 2;
        }

        Ok(MerkleProof {
            leaf,
            leaf_index,
            path,
            indices,
        })
    }

    /// Gets the hash value of a node at a specific level and index.
    ///
    /// For levels below the current tree depth, this reconstructs the hash.
    /// Empty positions return the zero hash for that level.
    fn get_node_at(&self, level: u8, index: u32) -> u128 {
        if level == 0 {
            // Leaf level
            if (index as usize) < self.leaves.len() {
                return self.leaves[index as usize];
            } else {
                return 0; // zeros(0) = 0
            }
        }

        // Check if this subtree is completely empty
        // A subtree at (level, index) covers leaf indices from 
        // index * 2^level to (index+1) * 2^level - 1
        let leaves_per_subtree = 1u32 << level;
        let subtree_start = index * leaves_per_subtree;
        
        // If all leaves in this subtree would be beyond our current tree size,
        // return the precomputed zero value
        if subtree_start >= self.next_index {
            return self.zeros(level);
        }

        // Otherwise compute by combining children
        let left_index = index * 2;
        let right_index = left_index + 1;

        let left = self.get_node_at(level - 1, left_index);
        let right = self.get_node_at(level - 1, right_index);

        self.hash_left_right(left, right)
    }
}

#[cfg(feature = "borsh")]
mod borsh_impl {
    // Note: Full borsh implementation would go here
    // For now, we document that this is available under the feature flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = MerkleTree::new(20).unwrap();
        assert_eq!(tree.levels(), 20);
        assert_eq!(tree.capacity(), 1 << 20);
        assert_eq!(tree.len(), 0);
        assert!(tree.is_empty());
    }

    #[test]
    fn test_new_tree_invalid_levels() {
        assert!(MerkleTree::new(0).is_err());
        assert!(MerkleTree::new(33).is_err());
    }

    #[test]
    fn test_insert_single() {
        let mut tree = MerkleTree::new(20).unwrap();
        let index = tree.insert(12345).unwrap();
        assert_eq!(index, 0);
        assert_eq!(tree.len(), 1);
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_insert_multiple() {
        let mut tree = MerkleTree::new(20).unwrap();
        for i in 0..10 {
            let index = tree.insert(i as u128).unwrap();
            assert_eq!(index, i);
        }
        assert_eq!(tree.len(), 10);
    }

    #[test]
    fn test_tree_full() {
        let mut tree = MerkleTree::new(2).unwrap(); // Can hold 4 leaves
        for i in 0..4 {
            tree.insert(i as u128).unwrap();
        }
        let result = tree.insert(100);
        assert!(matches!(result, Err(Error::TreeFull { .. })));
    }

    #[test]
    fn test_root_changes_on_insert() {
        let mut tree = MerkleTree::new(20).unwrap();
        let root1 = tree.root().unwrap();
        tree.insert(12345).unwrap();
        let root2 = tree.root().unwrap();
        assert_ne!(root1, root2);
    }

    #[test]
    fn test_is_known_root() {
        let mut tree = MerkleTree::new(20).unwrap();
        let root1 = tree.root().unwrap();
        tree.insert(12345).unwrap();
        let root2 = tree.root().unwrap();

        assert!(tree.is_known_root(root1));
        assert!(tree.is_known_root(root2));
        assert!(!tree.is_known_root(99999));
        assert!(!tree.is_known_root(0));
    }

    #[test]
    fn test_zeros_computation() {
        let tree = MerkleTree::new(10).unwrap();
        let zero0 = tree.zeros(0);
        let zero1 = tree.zeros(1);
        assert_eq!(zero0, 0);
        assert_ne!(zero1, 0);
    }

    #[test]
    fn test_deterministic_roots() {
        let mut tree1 = MerkleTree::new(10).unwrap();
        let mut tree2 = MerkleTree::new(10).unwrap();

        tree1.insert(123).unwrap();
        tree1.insert(456).unwrap();

        tree2.insert(123).unwrap();
        tree2.insert(456).unwrap();

        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_prove_valid_index() {
        let mut tree = MerkleTree::new(10).unwrap();
        tree.insert(12345).unwrap();
        tree.insert(67890).unwrap();

        let proof = tree.prove(0).unwrap();
        assert_eq!(proof.leaf, 12345);
        assert_eq!(proof.leaf_index, 0);
        assert_eq!(proof.path.len(), 10);
    }

    #[test]
    fn test_prove_invalid_index() {
        let mut tree = MerkleTree::new(10).unwrap();
        tree.insert(12345).unwrap();

        let result = tree.prove(1);
        assert!(matches!(result, Err(Error::LeafIndexOutOfBounds { .. })));
    }

    #[test]
    fn test_proof_verifies() {
        let mut tree = MerkleTree::new(10).unwrap();
        tree.insert(12345).unwrap();
        tree.insert(67890).unwrap();
        tree.insert(11111).unwrap();

        let root = tree.root().unwrap();

        for i in 0..3 {
            let proof = tree.prove(i).unwrap();
            assert!(proof.verify(root, &tree.hasher()), "Proof failed for leaf {}", i);
        }
    }
}
