use std::collections::HashMap;
use std::str::FromStr;

use cosmwasm_std::Uint256;

use crate::hasher::Hasher;
use crate::utils::{self, SolanaError};

pub const ROOT_HISTORY_SIZE: u8 = 30;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    levels: u8,
    filled_subtrees: HashMap<u8, Uint256>,
    roots: HashMap<u8, Uint256>,
    current_root_index: u8,
    next_index: u8
}

impl MerkleTree {
    pub fn new(levels: u8) -> Self {
        let mut instance = MerkleTree {
            levels,
            filled_subtrees: HashMap::new(),
            roots: HashMap::new(),
            current_root_index: 0,
            next_index: 0
        };

        for i in 0..levels {
            instance.filled_subtrees.insert(i, Self::zeros(i));
        }

        instance.roots.insert(0, Self::zeros(levels - 1));
        instance
    }

    pub fn root_hash(&self) -> Option<&Uint256> {
        self.roots.get(&self.current_root_index)
    }

    pub fn hash_left_right(&self, left: Uint256, right: Uint256) -> Uint256 {
        let field_size: Uint256 = Uint256::from_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").expect("Failed to parse field size");

        let mut r = left;
        let c = Uint256::zero();

        r = Hasher::mimc_sponge(&r, &c, &field_size);        
        r = r.checked_add(right).unwrap() % field_size;
        r = Hasher::mimc_sponge(&r, &c, &field_size);

        r
    }

    pub fn insert(&mut self, leaf: Uint256) -> Result<u8, SolanaError> {
        // if (self.next_index as usize) < 2_usize.pow(self.levels.into()) {
        //     return Err(utils::err("Merkle tree is full, no more leaves can be added").into());
        // }

        let _next_index = self.next_index;
        let mut current_index = self.next_index;
        let mut current_level_hash = leaf.clone();
        let mut left: Uint256;
        let mut right: Uint256;

        for i in 0..self.levels {
            if current_index % 2 == 0 {
                left = current_level_hash.clone();
                right = Self::zeros(i);
                self.filled_subtrees.insert(i, current_level_hash.clone());
            } else {
                left = self.filled_subtrees.get(&i).unwrap().clone();
                right = current_level_hash.clone();
            }
            current_level_hash = self.hash_left_right(left, right);
            current_index /= 2;
        }

        let new_root_index: u8 = (self.current_root_index + 1) % ROOT_HISTORY_SIZE;
        self.current_root_index = new_root_index;
        self.roots.insert(new_root_index, current_level_hash.clone());
        self.next_index = _next_index + 1;

        Ok(_next_index)
    }

    pub fn is_known_root(&self, root: Uint256) -> bool {
        if root.is_zero() {
            return false;
        }
    
        let current_root_index = self.current_root_index;
        let mut i = current_root_index;
        
        loop {
            if self.roots.get(&i).is_some() && *self.roots.get(&i).unwrap() == root {
                return true;
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

    pub fn get_last_root(&self) -> Uint256 {
        return self.roots.get(&self.current_root_index).unwrap().clone();
    }

    pub fn zeros(i: u8) -> Uint256 {
        let mut result = Uint256::from_u128(0);
        for _ in 0..i {
            result = Hasher::mimc_sponge(
                &result, 
                &Uint256::zero(),
                &Uint256::from_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").expect("Failed to parse field size")
            );
        }
        result
    }
}

impl ToString for MerkleTree {
    fn to_string(&self) -> String {
        let mut string_representation = String::new();
        
        string_representation.push_str(&format!("levels: {}\n", self.levels));
        
        string_representation.push_str("filled_subtrees:\n");
        for (level, value) in &self.filled_subtrees {
            string_representation.push_str(&format!("  {}: {}\n", level, value.to_string()));
        }
        
        string_representation.push_str("roots:\n");
        for (level, value) in &self.roots {
            string_representation.push_str(&format!("  {}: {}\n", level, value.to_string()));
        }
        
        string_representation.push_str(&format!("current_root_index: {}\n", self.current_root_index));
        string_representation.push_str(&format!("next_index: {}\n", self.next_index));
        
        string_representation
    }
}

impl FromStr for MerkleTree {
    type Err = SolanaError;

    fn from_str(s: &str) -> std::result::Result<Self, SolanaError> {
        let mut levels: Option<u8> = None;
        let mut filled_subtrees: HashMap<u8, Uint256> = HashMap::new();
        let mut roots: HashMap<u8, Uint256> = HashMap::new();
        let mut current_root_index: Option<u8> = None;
        let mut next_index: Option<u8> = None;

        for line in s.lines() {
            let parts: Vec<&str> = line.trim().splitn(2, ":").collect();
            if parts.len() != 2 {
                return Err(utils::err("Error").into()
                );
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
                        return Err(utils::err("Error occured in filled subtrees").into());
                    }
                    let level: u8 = level_value[0].trim().parse().map_err(|e| format!("Parsing filled_subtrees level failed: {}", e)).unwrap();
                    let value: Uint256 = level_value[1].trim().parse().map_err(|e| format!("Parsing filled_subtrees value failed: {}", e)).unwrap();
                    filled_subtrees.insert(level, value);
                }
                "roots" => {
                    let level_value: Vec<&str> = value.splitn(2, ":").collect();
                    if level_value.len() != 2 {
                        return Err(utils::err("Error in roots").into());
                    }
                    let level: u8 = level_value[0].trim().parse().map_err(|e| format!("Parsing roots level failed: {}", e)).unwrap();
                    let value: Uint256 = level_value[1].trim().parse().map_err(|e| format!("Parsing roots value failed: {}", e)).unwrap();
                    roots.insert(level, value);
                }
                "current_root_index" => {
                    current_root_index = Some(value.parse().map_err(|e| format!("Parsing current_root_index failed: {}", e)).unwrap());
                }
                "next_index" => {
                    next_index = Some(value.parse().map_err(|e| format!("Parsing next_index failed: {}", e)).unwrap());
                }
                _ => {
                    return Err(utils::err("Unexpected error").into());
                }
            }
        }

        let levels = levels.ok_or("Missing levels").unwrap();
        let current_root_index = current_root_index.ok_or("Missing current_root_index").unwrap();
        let next_index = next_index.ok_or("Missing next_index").unwrap();

        Ok(MerkleTree {
            levels,
            filled_subtrees,
            roots,
            current_root_index,
            next_index
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MERKLE_TREE_HEIGHT: u8 = 20;

    #[test]
    fn test_insert() {
        let mut merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        let leaf = Uint256::from_u128(123);
        let result = merkle_tree.insert(leaf);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_known_root() {
        let merkle_tree = MerkleTree::new(MERKLE_TREE_HEIGHT);
        let root = Uint256::from_u128(123);
        let result = merkle_tree.is_known_root(root);
        assert_eq!(result, false);
    }
}