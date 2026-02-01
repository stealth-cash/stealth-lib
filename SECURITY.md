# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.x     | ✅ Yes             |
| < 1.0   | ❌ No              |

## Reporting a Vulnerability

**Please DO NOT file public GitHub issues for security vulnerabilities.**

### How to Report

1. **Email**: Send details to `security@YOUR_DOMAIN.com`
2. **GitHub Security Advisories**: Use the "Report a vulnerability" button on the Security tab

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Initial Response**: Within 72 hours
- **Status Update**: Within 7 days
- **Resolution Target**: Within 30 days (severity-dependent)

### Process

1. We will acknowledge receipt of your report
2. We will investigate and validate the issue
3. We will develop and test a fix
4. We will release a patched version
5. We will publicly disclose the issue (coordinated with you)
6. We will credit you in the advisory (unless you prefer anonymity)

## Security Considerations

### ⚠️ Important Disclaimers

1. **NOT AUDITED**: This library has NOT been professionally audited. Use at your own risk.

2. **NOT CONSTANT-TIME**: The MiMC implementation is NOT constant-time and is vulnerable to timing side-channel attacks. Do not use in contexts where timing attacks are a concern.

3. **EXPERIMENTAL CODE**: The `experimental` feature flag enables educational code that is intentionally insecure. NEVER use in production.

### Known Limitations

| Component | Limitation | Mitigation |
|-----------|------------|------------|
| MiMC Hash | Not constant-time | Only use for ZK circuits where timing is not a concern |
| Merkle Tree | Not optimized for very large trees | Use sparse tree implementations for >2^20 leaves |
| Field Arithmetic | Uses `wrapping_*` operations | Verify compatibility with your ZK circuit field |

### Threat Model

#### In Scope
- Collision resistance of MiMC hash
- Correctness of Merkle proofs
- Memory safety (enforced by Rust + `#![deny(unsafe_code)]`)

#### Out of Scope
- Side-channel attacks (timing, power analysis)
- Denial of service (large inputs may be slow)
- Physical attacks
- Quantum resistance

### Best Practices

#### Do ✅
- Pin to specific versions in production
- Run `cargo audit` regularly
- Use established crates for non-ZK cryptography
- Validate all inputs before passing to library functions

#### Don't ❌
- Use `experimental` feature in production
- Use MiMC for password hashing
- Assume constant-time execution
- Ignore deprecation warnings

## Dependency Security

We aim to minimize dependencies, but we do use:

| Dependency | Purpose | Audited? |
|------------|---------|----------|
| `hex` | Hex encoding | Widely used, minimal |

Optional dependencies:
| Dependency | Purpose | Feature Flag |
|------------|---------|--------------|
| `serde` | Serialization | `serde` |
| `borsh` | Solana serialization | `borsh` |

Run `cargo audit` to check for known vulnerabilities in dependencies.

## Changelog

Security-related changes are documented in [CHANGELOG.md](CHANGELOG.md) under the "Security" heading.
