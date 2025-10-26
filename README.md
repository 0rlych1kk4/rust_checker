# rust_checker

[![Crates.io](https://img.shields.io/crates/v/rust_checker.svg)](https://crates.io/crates/rust_checker)
[![Downloads](https://img.shields.io/crates/d/rust_checker)](https://crates.io/crates/rust_checker)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A modular Rust code validation tool with HTML, JSON, SVG badge, and JUnit XML report export.

---

##  Installation

```bash
cargo install rust_checker
```

## Usage

```bash
rust_checker . --check-fmt --check-clippy
```

---

##  Post-Quantum Cryptography (PQC) Hybrid Example

Starting from version **v1.0.1**, `rust_checker` includes a real-world example demonstrating how to combine **post-quantum key encapsulation (Kyber/ML-KEM)** with **classical symmetric encryption (ChaCha20-Poly1305)** and **key derivation (HKDF-SHA256)** using production-grade Rust crypto libraries.

###  Overview
- **Post-Quantum KEM:** [pqcrypto-kyber](https://crates.io/crates/pqcrypto-kyber)
- **Key Derivation:** [ring::hkdf](https://crates.io/crates/ring)
- **Symmetric AEAD:** [ring::aead::CHACHA20_POLY1305](https://crates.io/crates/ring)
- **Secure RNG:** [rand::rngs::OsRng](https://crates.io/crates/rand)
- **Zeroization:** [zeroize](https://crates.io/crates/zeroize)
- **Constant-Time Comparison:** [subtle](https://crates.io/crates/subtle)

###  Example Flow
1. Generate a Kyber keypair  
2. Encapsulate/decapsulate to exchange a shared secret  
3. Derive a symmetric key via HKDF-SHA256  
4. Encrypt/decrypt a plaintext message with ChaCha20-Poly1305  
5. Zeroize secrets on drop  

### Run the Example
```bash
cargo fmt --all
cargo test --all-features
cargo run --bin rust_checker -- --run-pqc-hybrid

quit

---

##  Post-Quantum Cryptography (PQC) Hybrid Example

Starting from version **v1.0.1**, `rust_checker` includes a real-world example demonstrating how to combine **post-quantum key encapsulation (Kyber/ML-KEM)** with **classical symmetric encryption (ChaCha20-Poly1305)** and **key derivation (HKDF-SHA256)** using production-grade Rust crypto libraries.

###  Overview
- **Post-Quantum KEM:** [pqcrypto-kyber](https://crates.io/crates/pqcrypto-kyber)
- **Key Derivation:** [ring::hkdf](https://crates.io/crates/ring)
- **Symmetric AEAD:** [ring::aead::CHACHA20_POLY1305](https://crates.io/crates/ring)
- **Secure RNG:** [rand::rngs::OsRng](https://crates.io/crates/rand)
- **Zeroization:** [zeroize](https://crates.io/crates/zeroize)
- **Constant-Time Comparison:** [subtle](https://crates.io/crates/subtle)

###  Example Flow
1. Generate a Kyber keypair  
2. Encapsulate/decapsulate to exchange a shared secret  
3. Derive a symmetric key via HKDF-SHA256  
4. Encrypt/decrypt a plaintext message with ChaCha20-Poly1305  
5. Zeroize secrets on drop  

### Run the Example
```bash
cargo fmt --all
cargo test --all-features
cargo run --bin rust_checker -- --run-pqc-hybrid
