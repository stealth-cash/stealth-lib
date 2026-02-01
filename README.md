# stealth-lib

[![Crates.io](https://img.shields.io/crates/v/stealth-lib.svg)](https://crates.io/crates/stealth-lib)
[![Documentation](https://docs.rs/stealth-lib/badge.svg)](https://docs.rs/stealth-lib)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/stealth-cash/stealth-lib/workflows/CI/badge.svg)](https://github.com/stealth-cash/stealth-lib/actions)

ZK-friendly cryptographic primitives for Rust.

## Features

- **MiMC Hash**: Efficient hash function designed for ZK circuits (compatible with Tornado Cash / circomlib)
- **Merkle Tree**: MiMC-based tree with proof generation and verification
- **No unsafe code**: `#![deny(unsafe_code)]`
- **`no_std` support**: Optional, for WASM/embedded targets
- **Well-documented**: Comprehensive API documentation with examples

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
stealth-lib = "1.0"
```

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | ✅ | Enable standard library support |
| `serde` | ❌ | Enable serde serialization |
| `borsh` | ❌ | Enable borsh serialization (for Solana) |
| `experimental` | ❌ | ⚠️ Educational code only, NOT for production |

## Quick Start

### Merkle Tree with Proofs

```rust
use stealth_lib::{MerkleTree, MerkleProof};

fn main() -> stealth_lib::Result<()> {
    // Create a tree with 20 levels (can hold ~1M leaves)
    let mut tree = MerkleTree::new(20)?;

    // Insert some leaves (e.g., commitment hashes)
    let commitment = 12345u128;
    let index = tree.insert(commitment)?;

    // Generate a proof for the commitment
    let proof = tree.prove(index)?;

    // Verify the proof against the current root
    let root = tree.root().unwrap();
    assert!(proof.verify(root, &tree.hasher()));

    println!("Commitment {} proven at index {}", commitment, index);
    Ok(())
}
```

### MiMC Hashing

```rust
use stealth_lib::hash::MimcHasher;

fn main() {
    let hasher = MimcHasher::default();

    // Hash two values (e.g., for a commitment)
    let nullifier = 123456789u128;
    let secret = 987654321u128;
    let commitment = hasher.hash(nullifier, secret);

    println!("Commitment: {}", commitment);
}
```

## Security Model

### Designed For
- Zero-knowledge proof circuits (Tornado Cash, Semaphore, etc.)
- On-chain verification of Merkle membership proofs
- Privacy-preserving applications using ZK-SNARKs

### Guarantees
- ✅ Collision resistance of MiMC (computational security)
- ✅ Correct Merkle proofs for membership verification
- ✅ Deterministic outputs for same inputs
- ✅ Root history buffer for handling concurrent insertions

### Non-Goals / Explicit Exclusions
- ❌ **NOT constant-time** — Vulnerable to timing side-channels
- ❌ **NOT a general-purpose crypto library** — Use `ring`, `sha2`, etc.
- ❌ **NOT professionally audited** — Use at your own risk
- ❌ **NOT suitable for password hashing** — Use argon2, bcrypt, scrypt

### Do ✅
- Use for building ZK circuits
- Verify proofs on-chain (Solana, Ethereum)
- Use established libraries for non-ZK crypto

### Don't ❌
- Use MiMC for password hashing
- Use the `experimental` feature in production
- Assume constant-time execution
- Use for cryptographic signatures

## API Overview

### Core Types

| Type | Description |
|------|-------------|
| `MerkleTree` | Sparse Merkle tree with MiMC hash |
| `MerkleProof` | Merkle inclusion proof |
| `MimcHasher` | MiMC-Feistel sponge hasher |
| `Error` | Typed error enum |
| `Result<T>` | Result alias with `Error` |

### Error Handling

All fallible operations return `Result<T, Error>`:

```rust
use stealth_lib::{MerkleTree, Error};

let tree = MerkleTree::new(0);
assert!(matches!(tree, Err(Error::InvalidTreeConfig(_))));

let mut tree = MerkleTree::new(2).unwrap(); // 4 leaves max
for _ in 0..4 {
    tree.insert(0).unwrap();
}
let result = tree.insert(0);
assert!(matches!(result, Err(Error::TreeFull { .. })));
```

## Migration from v0.x

Version 1.0 introduces breaking changes for improved safety:

```rust
// Old (v0.x)
use stealth_lib::merkle_tree::MerkleTree;
let tree = MerkleTree::new(20);  // Could panic
let root = tree.get_last_root(); // Could panic

// New (v1.0)
use stealth_lib::MerkleTree;
let tree = MerkleTree::new(20).unwrap();  // Returns Result
let root = tree.root().unwrap();          // Returns Option
```

See [CHANGELOG.md](CHANGELOG.md) for full migration guide.

## MSRV

Minimum Supported Rust Version: **1.70.0**

## Benchmarks

Run benchmarks with:

```bash
cargo bench
```

Typical results on modern hardware:
- `mimc_hash`: ~500ns
- `merkle_insert (depth 20)`: ~50μs
- `merkle_prove (depth 20)`: ~100μs
- `merkle_verify (depth 20)`: ~50μs

## Contributing

Contributions are welcome! Please open an issue or PR.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Security

For security issues, please see [SECURITY.md](SECURITY.md).