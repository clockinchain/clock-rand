//! Cryptographically secure RNGs
//!
//! This module contains RNGs suitable for cryptographic operations
//! such as key generation, nonce generation, and signature operations.

#[cfg(feature = "crypto_rng")]
pub mod aes_ctr;
#[cfg(feature = "crypto_rng")]
pub mod blake3_drbg;
#[cfg(feature = "crypto_rng")]
pub mod chacha20;

#[cfg(feature = "crypto_rng")]
pub use aes_ctr::AesCtrRng;
#[cfg(feature = "crypto_rng")]
pub use blake3_drbg::Blake3Drbg;
#[cfg(feature = "crypto_rng")]
pub use chacha20::ChaCha20Rng;
