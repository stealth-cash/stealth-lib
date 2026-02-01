# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-02-01

### Added

- **New module structure**:
  - `hash::MimcHasher` - Configurable MiMC hash function
  - `merkle::MerkleTree` - Improved Merkle tree with proof support
  - `merkle::MerkleProof` - Merkle inclusion proofs
  - `error::Error` - Typed error enum
  - `encoding` - Hex encoding utilities

- **Merkle proof generation and verification**:
  ```rust
  let proof = tree.prove(leaf_index)?;
  assert!(proof.verify(root, &tree.hasher()));
  ```

- **Typed error handling** - All operations return `Result<T, Error>`

- **Feature flags**:
  - `std` (default) - Standard library support
  - `serde` - Serde serialization
  - `borsh` - Borsh serialization for Solana
  - `experimental` - Educational crypto code (DO NOT USE IN PRODUCTION)

- **`no_std` support** - Disable `std` feature for embedded/WASM

- **Comprehensive documentation** with examples for all public APIs

- **Benchmarks** using Criterion

### Changed

- **BREAKING**: `MerkleTree::new()` now returns `Result<Self, Error>`
  ```rust
  // Before
  let tree = MerkleTree::new(20);
  
  // After  
  let tree = MerkleTree::new(20)?;
  ```

- **BREAKING**: `MerkleTree::insert()` returns `Result<u32, Error>` (was `Result<u8, SolanaError>`)
  - Index type changed from `u8` to `u32` to support larger trees

- **BREAKING**: Error type renamed from `SolanaError` to `Error`
  - Migration: `use stealth_lib::Error;`

- **BREAKING**: `Hasher` renamed to `MimcHasher` and moved to `hash` module
  - Migration: `use stealth_lib::hash::MimcHasher;`

- `borsh` dependency is now optional (enable with `borsh` feature)

- `primitive-types` dependency removed

### Deprecated

- `stealth_lib::hasher::Hasher` - Use `MimcHasher` instead
- `stealth_lib::merkle_tree::MerkleTree` - Use `stealth_lib::MerkleTree` instead
- `stealth_lib::utils::SolanaError` - Use `stealth_lib::Error` instead
- `MerkleTree::get_last_root()` - Use `MerkleTree::root()` instead

### Security

- Added `#![deny(unsafe_code)]` crate-wide
- Removed all `expect()`/`unwrap()` from non-test code paths
- All user-facing errors are typed (no panics for invalid input)
- Added `#![warn(missing_docs)]` for documentation coverage

### Migration Guide

#### Error Handling

```rust
// v0.x - Could panic
let tree = MerkleTree::new(20);
let root = tree.get_last_root();

// v1.0 - Explicit error handling
let tree = MerkleTree::new(20)?;
let root = tree.root().ok_or("No root")?;
```

#### Import Changes

```rust
// v0.x
use stealth_lib::hasher::Hasher;
use stealth_lib::merkle_tree::MerkleTree;
use stealth_lib::utils::SolanaError;

// v1.0
use stealth_lib::{MimcHasher, MerkleTree, Error};
// or
use stealth_lib::hash::MimcHasher;
use stealth_lib::merkle::MerkleTree;
use stealth_lib::error::Error;
```

#### Feature Flags

```toml
# v0.x - borsh always included
[dependencies]
stealth-lib = "0.1"

# v1.0 - borsh is optional
[dependencies]
stealth-lib = { version = "1.0", features = ["borsh"] }
```

## [0.1.4] - Previous Release

- Initial release with MiMC hasher and Merkle tree
- Basic ZKP utilities
