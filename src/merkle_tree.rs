//! Legacy Merkle tree module for backwards compatibility.
//!
//! # Deprecated
//!
//! This module is deprecated. Use [`crate::merkle::MerkleTree`] instead.
//!
//! ## Migration
//!
//! ```ignore
//! // Old code:
//! use stealth_lib::merkle_tree::MerkleTree;
//! let tree = MerkleTree::new(20);
//!
//! // New code:
//! use stealth_lib::MerkleTree;
//! let tree = MerkleTree::new(20).unwrap();
//! ```

#![allow(missing_docs)]
#![allow(deprecated)]

use std::collections::HashMap;
use std::str::FromStr;

use crate::hash::MimcHasher;
#[allow(deprecated)]
use crate::utils::{self, SolanaError};

pub const ROOT_HISTORY_SIZE: u8 = 30;

/// Legacy MerkleTree implementation.
///
/// # Deprecated
///
/// Use [`crate::merkle::MerkleTree`] instead, which provides:
/// - Proper error handling (no panics)
/// - Proof generation and verification
/// - Larger tree support (u32 indices instead of u8)
#[deprecated(since = "1.0.0", note = "Use crate::merkle::MerkleTree instead")]
#[derive(Debug, Clone)]
pub struct MerkleTree {
    levels: u8,
    filled_subtrees: HashMap<u8, u128>,
    roots: HashMap<u8, u128>,
    current_root_index: u8,
    next_index: u8,
    hasher: MimcHasher,
}

#[allow(deprecated)]
impl MerkleTree {
    pub fn new(levels: u8) -> Self {
        let hasher = MimcHasher::default();
        let mut instance = MerkleTree {
            levels,
            filled_subtrees: HashMap::new(),
            roots: HashMap::new(),
            current_root_index: 0,
            next_index: 0,
            hasher,
        };

        for i in 0..levels {
            instance.filled_subtrees.insert(i, Self::zeros(i));
        }

        instance.roots.insert(0, Self::zeros(levels - 1));
        instance
    }

    pub fn root_hash(&self) -> Option<&u128> {
        self.roots.get(&self.current_root_index)
    }

    pub fn hash_left_right(&self, left: u128, right: u128) -> u128 {
        let field_size = self.hasher.field_prime();
        let c = 0_u128;

        let mut r = left;
        r = self.hasher.mimc_sponge(r, c, field_size);
        r = r.wrapping_add(right) % field_size;
        r = self.hasher.mimc_sponge(r, c, field_size);

        r
    }

    pub fn insert(&mut self, leaf: u128) -> Result<u8, SolanaError> {
        let _next_index = self.next_index;
        let mut current_index = self.next_index;
        let mut current_level_hash = leaf;
        let mut left: u128;
        let mut right: u128;

        for i in 0..self.levels {
            if current_index % 2 == 0 {
                left = current_level_hash;
                right = Self::zeros(i);
                self.filled_subtrees.insert(i, current_level_hash);
            } else {
                left = *self.filled_subtrees.get(&i).unwrap_or(&Self::zeros(i));
                right = current_level_hash;
            }
            current_level_hash = self.hash_left_right(left, right);
            current_index /= 2;
        }

        let new_root_index: u8 = (self.current_root_index + 1) % ROOT_HISTORY_SIZE;
        self.current_root_index = new_root_index;
        self.roots.insert(new_root_index, current_level_hash);
        self.next_index = _next_index + 1;

        Ok(_next_index)
    }

    pub fn is_known_root(&self, root: u128) -> bool {
        if root == 0 {
            return false;
        }
    
        let current_root_index = self.current_root_index;
        let mut i = current_root_index;
        
        loop {
            if let Some(&stored_root) = self.roots.get(&i) {
                if stored_root == root {
                    return true;
                }
            }
            if i == 0 {
                i = ROOT_HISTORY_SIZE - 1;
            } else {
                i -= 1;
            }
            if i == current_root_index {
                break;
            }
        }
        false
    }

    pub fn get_last_root(&self) -> u128 {
        *self.roots.get(&self.current_root_index).unwrap()
    }

    pub fn zeros(i: u8) -> u128 {
        let hasher = MimcHasher::default();
        let mut result = 0;
        for _ in 0..i {
            result = hasher.mimc_sponge(result, 0, hasher.field_prime());
        }
        result
    }
}

#[allow(deprecated)]
impl std::fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "levels: {}", self.levels)?;
        
        writeln!(f, "filled_subtrees:")?;
        for (level, value) in &self.filled_subtrees {
            writeln!(f, "  {}: {}", level, value)?;
        }
        
        writeln!(f, "roots:")?;
        for (level, value) in &self.roots {
            writeln!(f, "  {}: {}", level, value)?;
        }
        
        writeln!(f, "current_root_index: {}", self.current_root_index)?;
        write!(f, "next_index: {}", self.next_index)
    }
}

#[allow(deprecated)]
impl FromStr for MerkleTree {
    type Err = SolanaError;

    fn from_str(s: &str) -> std::result::Result<Self, SolanaError> {
        let mut levels: Option<u8> = None;
        let mut filled_subtrees: HashMap<u8, u128> = HashMap::new();
        let mut roots: HashMap<u8, u128> = HashMap::new();
        let mut current_root_index: Option<u8> = None;
        let mut next_index: Option<u8> = None;

        for line in s.lines() {
            let parts: Vec<&str> = line.trim().splitn(2, ":").collect();
            if parts.len() != 2 {
                return Err(utils::err("Error"));
            }
            let key = parts[0].trim();
            let value = parts[1].trim();

            match key {
                "levels" => {
                    levels = Some(value.parse().map_err(|e| format!("Parsing levels failed: {}", e)).unwrap());
                }
                "filled_subtrees" => {
                    let level_value: Vec<&str> = value.splitn(2, ":").collect();
                    if level_value.len() != 2 {
                        return Err(utils::err("Error occured in filled subtrees"));
                    }
                    let level: u8 = level_value[0].trim().parse().map_err(|e| format!("Parsing filled_subtrees level failed: {}", e)).unwrap();
                    let value: u128 = level_value[1].trim().parse().map_err(|e| format!("Parsing filled_subtrees value failed: {}", e)).unwrap();
                    filled_subtrees.insert(level, value);
                }
                "roots" => {
                    let level_value: Vec<&str> = value.splitn(2, ":").collect();
                    if level_value.len() != 2 {
                        return Err(utils::err("Error in roots"));
                    }
                    let level: u8 = level_value[0].trim().parse().map_err(|e| format!("Parsing roots level failed: {}", e)).unwrap();
                    let value: u128 = level_value[1].trim().parse().map_err(|e| format!("Parsing roots value failed: {}", e)).unwrap();
                    roots.insert(level, value);
                }
                "current_root_index" => {
                    current_root_index = Some(value.parse().map_err(|e| format!("Parsing current_root_index failed: {}", e)).unwrap());
                }
                "next_index" => {
                    next_index = Some(value.parse().map_err(|e| format!("Parsing next_index failed: {}", e)).unwrap());
                }
                _ => {
                    return Err(utils::err("Unexpected error"));
                }
            }
        }

        let levels = levels.ok_or_else(|| utils::err("Missing levels")).unwrap();
        let current_root_index = current_root_index.ok_or_else(|| utils::err("Missing current_root_index")).unwrap();
        let next_index = next_index.ok_or_else(|| utils::err("Missing next_index")).unwrap();

        Ok(MerkleTree {
            levels,
            filled_subtrees,
            roots,
            current_root_index,
            next_index,
            hasher: MimcHasher::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MERKLE_TREE_HEIGHT: u8 = 20;

    #[test]
    #[allow(deprecated)]
    fn test_insert() {
        let mut merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        let leaf = 123;
        let result = merkle_tree.insert(leaf);
        assert!(result.is_ok());
    }

    #[test]
    #[allow(deprecated)]
    fn test_is_known_root() {
        let merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        let root = 123;
        let result = merkle_tree.is_known_root(root);
        assert_eq!(result, false);
    }
}