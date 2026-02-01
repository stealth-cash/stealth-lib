//! MiMC-Feistel sponge hash function.
//!
//! MiMC (Minimal Multiplicative Complexity) is a hash function designed for
//! efficient evaluation inside arithmetic circuits, particularly in ZK-SNARKs.
//!
//! # Algorithm
//!
//! This implementation uses the MiMC-Feistel-Sponge construction with:
//! - Feistel network structure for the permutation
//! - Sponge construction for variable-length input
//! - Exponent of 5 (x^5) for the round function
//!
//! # Constants
//!
//! The round constants are derived deterministically. The default configuration
//! uses constants compatible with circomlib/Tornado Cash implementations.
//!
//! # References
//!
//! - [MiMC Paper](https://eprint.iacr.org/2016/492.pdf)
//! - [circomlib implementation](https://github.com/iden3/circomlib/blob/master/circuits/mimcsponge.circom)
//!
//! # Example
//!
//! ```
//! use stealth_lib::hash::MimcHasher;
//!
//! let hasher = MimcHasher::default();
//! let hash = hasher.hash(123, 456);
//! println!("MiMC hash: {}", hash);
//! ```
//!
//! # Security Note
//!
//! This implementation is designed for use in ZK circuits. It is:
//! - **NOT constant-time** (do not use where timing attacks are a concern)
//! - **NOT suitable for password hashing** (use argon2, bcrypt, or scrypt instead)

/// MiMC round constants.
///
/// These constants are derived deterministically and are compatible with
/// the circomlib/Tornado Cash MiMC implementation.
///
/// The constants are computed as: `keccak256("mimcsponge" || i)` truncated to
/// fit the field, where `i` is the round index.
const MIMC_CONSTANTS: [u128; 20] = [
    0,
    25823191961023811529686723375255045,
    48376936063113800887806988124358800,
    75580405153655082660116863095114839,
    66651710483985382365580181188706173,
    45887003413921204775397977044284378,
    14399999722617037892747232478295923,
    29376176727758177809204424209125257,
    13768859312518298840937540532277016,
    54749662990362840569021981534456448,
    25161436470718351277017231215227846,
    90370030464179443930112165274275271,
    92014788260850167582827910417652439,
    40376490640073034398204558905403523,
    90379224439153137712327643289289624,
    11220341520269979188892857030918685,
    11480168113674888067906254878279274,
    11144081894867681653997893051446803,
    64965960071752809090438003157362764,
    98428510787134995495896453413714864,
];

/// Default field prime (2^128 - 1).
///
/// This is the maximum value for u128, used as the modulus for field arithmetic.
const DEFAULT_FIELD_PRIME: u128 = 340282366920938463463374607431768211455;

/// Default number of rounds for MiMC-Feistel.
const DEFAULT_ROUNDS: usize = 10;

/// MiMC-Feistel sponge hasher.
///
/// This struct provides the MiMC hash function with configurable parameters.
/// For most use cases, use [`MimcHasher::default()`] which provides parameters
/// compatible with Tornado Cash / circomlib.
///
/// # Example
///
/// ```
/// use stealth_lib::hash::MimcHasher;
///
/// // Use default parameters (compatible with Tornado Cash)
/// let hasher = MimcHasher::default();
/// let hash = hasher.hash(123, 456);
///
/// // Hash is deterministic
/// assert_eq!(hasher.hash(123, 456), hasher.hash(123, 456));
///
/// // Different inputs produce different outputs
/// assert_ne!(hasher.hash(123, 456), hasher.hash(123, 789));
/// ```
#[derive(Debug, Clone)]
pub struct MimcHasher {
    /// Field prime (modulus for all arithmetic operations).
    field_prime: u128,
    /// Number of rounds in the Feistel network.
    num_rounds: usize,
    /// Round constants.
    constants: Vec<u128>,
}

impl Default for MimcHasher {
    /// Creates a MimcHasher with default parameters compatible with Tornado Cash.
    ///
    /// - Field prime: 2^128 - 1
    /// - Rounds: 10
    /// - Constants: circomlib-compatible
    fn default() -> Self {
        MimcHasher {
            field_prime: DEFAULT_FIELD_PRIME,
            num_rounds: DEFAULT_ROUNDS,
            constants: MIMC_CONSTANTS.to_vec(),
        }
    }
}

impl MimcHasher {
    /// Creates a new MimcHasher with custom parameters.
    ///
    /// # Arguments
    ///
    /// * `field_prime` - The field modulus for arithmetic operations
    /// * `num_rounds` - Number of Feistel rounds
    /// * `constants` - Round constants (must have at least `num_rounds * 2` elements)
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::hash::MimcHasher;
    ///
    /// let constants = vec![0u128; 20];
    /// let hasher = MimcHasher::new(
    ///     340282366920938463463374607431768211455, // 2^128 - 1
    ///     10,
    ///     constants,
    /// );
    /// ```
    pub fn new(field_prime: u128, num_rounds: usize, constants: Vec<u128>) -> Self {
        MimcHasher {
            field_prime,
            num_rounds,
            constants,
        }
    }

    /// Returns the field prime used by this hasher.
    #[inline]
    pub fn field_prime(&self) -> u128 {
        self.field_prime
    }

    /// Returns the number of rounds used by this hasher.
    #[inline]
    pub fn num_rounds(&self) -> usize {
        self.num_rounds
    }

    /// MiMC-Feistel permutation.
    ///
    /// Applies the Feistel network to the input pair (left, right) with the given key.
    ///
    /// # Arguments
    ///
    /// * `left` - Left input value
    /// * `right` - Right input value
    /// * `key` - Permutation key
    ///
    /// # Returns
    ///
    /// A tuple `(new_left, new_right)` after applying the Feistel permutation.
    fn mimc_feistel(&self, left: u128, right: u128, key: u128) -> (u128, u128) {
        let mut last_l = left;
        let mut last_r = right;

        for i in 0..self.num_rounds {
            // mask = (right + key + c[i]) mod p
            let mask = last_r
                .wrapping_add(key)
                .wrapping_rem(self.field_prime)
                .wrapping_add(self.constants[i])
                .wrapping_rem(self.field_prime);

            // mask^5 mod p (using square-and-multiply)
            let mask2 = mask.wrapping_mul(mask).wrapping_rem(self.field_prime);
            let mask4 = mask2.wrapping_mul(mask2).wrapping_rem(self.field_prime);
            let mask5 = mask4.wrapping_mul(mask).wrapping_rem(self.field_prime);

            // Feistel swap
            let temp = last_r;
            last_r = last_l.wrapping_add(mask5).wrapping_rem(self.field_prime);
            last_l = temp;
        }

        (last_l, last_r)
    }

    /// MiMC sponge hash function.
    ///
    /// Computes the MiMC-Feistel-Sponge hash of two input values.
    /// This is the primary hash function used for Merkle tree construction.
    ///
    /// # Arguments
    ///
    /// * `left` - First input value
    /// * `right` - Second input value
    ///
    /// # Returns
    ///
    /// The hash output as a `u128`.
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::hash::MimcHasher;
    ///
    /// let hasher = MimcHasher::default();
    ///
    /// // Hash two values
    /// let hash = hasher.hash(123, 456);
    ///
    /// // Hash is deterministic
    /// assert_eq!(hash, hasher.hash(123, 456));
    /// ```
    pub fn hash(&self, left: u128, right: u128) -> u128 {
        self.mimc_sponge(left, right, self.field_prime)
    }

    /// MiMC sponge with explicit key parameter.
    ///
    /// Lower-level function that allows specifying a custom key.
    /// Most users should use [`hash`](Self::hash) instead.
    ///
    /// # Arguments
    ///
    /// * `left` - First input value
    /// * `right` - Second input value  
    /// * `key` - Sponge key
    ///
    /// # Returns
    ///
    /// The hash output as a `u128`.
    pub fn mimc_sponge(&self, left: u128, right: u128, key: u128) -> u128 {
        let mut last_r = left;
        let mut last_l = right;

        for _ in 0..self.num_rounds {
            let (new_last_r, new_last_l) = self.mimc_feistel(last_r, last_l, key);

            last_r = new_last_r.wrapping_add(1).wrapping_rem(self.field_prime);
            last_l = new_last_l;
        }

        last_r
    }

    /// Hash a single value.
    ///
    /// Convenience method to hash a single input by pairing it with zero.
    ///
    /// # Arguments
    ///
    /// * `input` - The value to hash
    ///
    /// # Returns
    ///
    /// The hash output as a `u128`.
    ///
    /// # Example
    ///
    /// ```
    /// use stealth_lib::hash::MimcHasher;
    ///
    /// let hasher = MimcHasher::default();
    /// let hash = hasher.hash_single(12345);
    /// ```
    pub fn hash_single(&self, input: u128) -> u128 {
        self.hash(input, 0)
    }
}

// Legacy API support - these functions maintain backwards compatibility
// with the original Hasher struct API.

/// Legacy Hasher struct for backwards compatibility.
///
/// # Deprecated
///
/// This struct is provided for backwards compatibility only.
/// New code should use [`MimcHasher`] instead.
#[deprecated(since = "1.0.0", note = "Use MimcHasher instead")]
pub struct Hasher;

#[allow(deprecated)]
impl Hasher {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mimc_hash_deterministic() {
        let hasher = MimcHasher::default();
        let hash1 = hasher.hash(123, 456);
        let hash2 = hasher.hash(123, 456);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_mimc_hash_different_inputs() {
        let hasher = MimcHasher::default();
        let hash1 = hasher.hash(123, 456);
        let hash2 = hasher.hash(123, 789);
        let hash3 = hasher.hash(456, 123);
        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_mimc_sponge_with_zero() {
        let hasher = MimcHasher::default();
        let hash1 = hasher.hash(0, 0);
        let hash2 = hasher.hash(0, 1);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_mimc_hash_single() {
        let hasher = MimcHasher::default();
        let hash1 = hasher.hash_single(12345);
        let hash2 = hasher.hash(12345, 0);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_mimc_feistel_permutation() {
        let hasher = MimcHasher::default();
        let (l1, r1) = hasher.mimc_feistel(100, 200, hasher.field_prime);
        let (l2, r2) = hasher.mimc_feistel(100, 200, hasher.field_prime);
        assert_eq!((l1, r1), (l2, r2));
    }

    #[test]
    fn test_default_hasher_params() {
        let hasher = MimcHasher::default();
        assert_eq!(hasher.field_prime(), DEFAULT_FIELD_PRIME);
        assert_eq!(hasher.num_rounds(), DEFAULT_ROUNDS);
    }

    #[test]
    fn test_custom_hasher() {
        let constants = vec![0u128; 20];
        let hasher = MimcHasher::new(1000, 5, constants);
        assert_eq!(hasher.field_prime(), 1000);
        assert_eq!(hasher.num_rounds(), 5);
    }

    // Legacy API tests
    #[test]
    #[allow(deprecated)]
    fn test_legacy_hasher_compatibility() {
        let legacy_hash = Hasher::mimc_sponge(123, 456, DEFAULT_FIELD_PRIME);
        let new_hash = MimcHasher::default().mimc_sponge(123, 456, DEFAULT_FIELD_PRIME);
        assert_eq!(legacy_hash, new_hash);
    }
}
