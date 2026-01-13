//! clock-rand: Custom blockchain-aware RNG crate
//!
//! Provides fast deterministic PRNGs, cryptographically secure RNGs,
//! and custom blockchain-aware RNGs by Olyntar Labs, an Olyntar company.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod error;
pub mod seed;
pub mod traits;

#[cfg(feature = "crypto_rng")]
pub mod crypto;
#[cfg(feature = "custom_rng")]
pub mod custom;
#[cfg(feature = "distributions")]
pub mod distributions;
#[cfg(feature = "fast_rng")]
pub mod fast;
pub mod utils;

#[cfg(feature = "serde")]
pub mod serialization;
#[cfg(feature = "thread_safe")]
pub mod thread_safe;
#[cfg(feature = "wasm")]
pub mod wasm;

// Re-export main types
pub use error::{EntropyError, Error, ForkError, Result, SeedError};
pub use seed::Seed;
pub use traits::{CryptoRng, DeterministicRng, Rng, RngCore, RngExt, SeedableRng};

// Re-export utility functions
pub use utils::{
    choose, fill_bytes, gen_f32, gen_f64, gen_range_f32, gen_range_f64, gen_range_u32,
    gen_range_u64, shuffle, weighted_choose,
};

#[cfg(feature = "std")]
pub use utils::sample;

#[cfg(feature = "fast_rng")]
pub use fast::{Pcg64, SplitMix64, Xoshiro256Plus};

#[cfg(feature = "crypto_rng")]
pub use crypto::{AesCtrRng, Blake3Drbg, ChaCha20Rng};

#[cfg(feature = "custom_rng")]
pub use custom::{ChainSeedX, EntroCrypt, HashMix256};

#[cfg(feature = "distributions")]
pub use distributions::{Distribution, Uniform};

// Convenience type aliases

/// Fast deterministic RNG type alias (Xoshiro256+)
pub type FastRng = Xoshiro256Plus;

/// Cryptographically secure RNG type alias
#[cfg(feature = "crypto_rng")]
pub type CryptoRngType = ChaCha20Rng;

/// Convenience constructor for fast RNG from blockchain state
#[cfg(feature = "fast_rng")]
pub fn fast_rng_from_blockchain(block_hash: &[u8; 32], timestamp: u64) -> Result<FastRng> {
    let seed = Seed::from_blockchain_state(block_hash, timestamp, None)?;
    Xoshiro256Plus::from_seed_obj(&seed)
}

/// Convenience constructor for crypto RNG from blockchain seed
#[cfg(feature = "crypto_rng")]
pub fn crypto_rng_from_seed(seed: &Seed) -> Result<ChaCha20Rng> {
    ChaCha20Rng::new(seed)
}

/// Convenience constructor for crypto RNG from OS entropy (requires std)
#[cfg(all(feature = "crypto_rng", feature = "std"))]
pub fn crypto_rng_from_os() -> Result<ChaCha20Rng> {
    ChaCha20Rng::from_os_entropy()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_rng_basic() {
        #[cfg(feature = "fast_rng")]
        {
            let mut rng = Xoshiro256Plus::new(42);
            let _ = rng.next_u64();
            assert!(rng.is_deterministic());
        }
    }

    #[test]
    fn test_crypto_rng_basic() {
        #[cfg(feature = "crypto_rng")]
        {
            let seed =
                Seed::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
                    .unwrap();
            let mut rng = ChaCha20Rng::new(&seed).unwrap();
            let _ = Rng::next_u64(&mut rng);
            assert!(rng.is_deterministic());
        }
    }
}
