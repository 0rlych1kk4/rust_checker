# Changelog

All notable changes to this project will be documented in this file.

---

## [1.0.1] - 2025-10-26
###  Stable Release Highlights
- Published as **stable version (v1.0.1)** of `rust_checker`  
- Verified across all 21 modules with **0 validation failures**  
- Fully **passes cargo fmt, clippy, and test checks**

###  New PQC Hybrid Example
A new real-world **Post-Quantum Cryptography (PQC)** hybrid demo is included, showing secure integration of:
- **Post-Quantum KEM:** Kyber (via `pqcrypto-kyber`)
- **Key Derivation:** HKDF-SHA256 (via `ring`)
- **Symmetric AEAD:** ChaCha20-Poly1305 (via `ring`)
- **Secure RNG:** OsRng (via `rand`)
- **Zeroization:** `zeroize` crate
- **Constant-Time Comparison:** `subtle` crate

**Usage:**
```bash
cargo run --bin rust_checker -- --run-pqc-hybrid

