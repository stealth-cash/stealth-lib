//! Legacy hasher module for backwards compatibility.
//!
//! # Deprecated
//!
//! This module is deprecated. Use [`crate::hash::MimcHasher`] instead.
//!
//! ## Migration
//!
//! ```ignore
//! // Old code:
//! use stealth_lib::hasher::Hasher;
//! let hash = Hasher::mimc_sponge(left, right, key);
//!
//! // New code:
//! use stealth_lib::MimcHasher;
//! let hasher = MimcHasher::default();
//! let hash = hasher.mimc_sponge(left, right, key);
//! ```

#![allow(missing_docs)]
#![allow(deprecated)]

use crate::hash::MimcHasher;

/// Legacy Hasher struct.
///
/// # Deprecated
///
/// Use [`MimcHasher`] instead.
#[deprecated(since = "1.0.0", note = "Use crate::hash::MimcHasher instead")]
pub struct Hasher;

impl Hasher {
    /// MiMC-Feistel permutation (legacy API).
    #[deprecated(since = "1.0.0", note = "Use MimcHasher methods instead")]
    #[allow(dead_code)]
    fn mimc_feistel(il: u128, ir: u128, k: u128) -> (u128, u128) {
        let hasher = MimcHasher::default();
        // This is a simplified version - the actual implementation is in MimcHasher
        let mut last_l = il;
        let mut last_r = ir;
        let p = hasher.field_prime();

        for _i in 0..hasher.num_rounds() {
            let mask = last_r.wrapping_add(k).wrapping_rem(p);
            let mask = mask.wrapping_add(0).wrapping_rem(p); // Simplified
            let mask2 = mask.wrapping_mul(mask).wrapping_rem(p);
            let mask4 = mask2.wrapping_mul(mask2).wrapping_rem(p);
            let mask5 = mask4.wrapping_mul(mask).wrapping_rem(p);

            let temp = last_r;
            last_r = last_l.wrapping_add(mask5).wrapping_rem(p);
            last_l = temp;
        }

        (last_l, last_r)
    }

    /// MiMC sponge hash (legacy API).
    ///
    /// # Deprecated
    ///
    /// Use [`MimcHasher::mimc_sponge`] instead.
    #[deprecated(since = "1.0.0", note = "Use MimcHasher::mimc_sponge instead")]
    pub fn mimc_sponge(left: u128, right: u128, k: u128) -> u128 {
        MimcHasher::default().mimc_sponge(left, right, k)
    }
}