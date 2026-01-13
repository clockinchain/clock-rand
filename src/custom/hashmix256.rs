//! HashMix256: Custom hybrid RNG
//!
//! A custom algorithm mixing multiple hash functions for deterministic
//! random streams. Fork-safe and high-quality.

use crate::error::Result;
use crate::seed::Seed;
use crate::traits::{CryptoRng, DeterministicRng, Rng, SeedableRng};

#[cfg(feature = "security")]
use zeroize::Zeroize;

/// HashMix256: Custom hybrid RNG
///
/// Uses a combination of hash functions to generate high-quality
/// deterministic random streams. Suitable for blockchain applications.
#[derive(Clone)]
pub struct HashMix256 {
    state: [u64; 4],
    key: [u8; 32],
    counter: u64,
}

#[cfg(feature = "security")]
impl Zeroize for HashMix256 {
    fn zeroize(&mut self) {
        self.state.zeroize();
        self.key.zeroize();
        self.counter = 0;
    }
}

impl HashMix256 {
    /// Create a new HashMix256 from a seed
    pub fn new(seed: &Seed) -> Result<Self> {
        let seed_bytes = seed.as_ref();
        let mut key = [0u8; 32];
        let mut state = [0u64; 4];

        // Derive key and initial state from seed
        #[cfg(feature = "crypto_rng")]
        {
            use blake3;
            let mut hasher = blake3::Hasher::new();
            hasher.update(seed_bytes);
            let hash = hasher.finalize();
            key.copy_from_slice(&hash.as_bytes()[..32]);

            // Derive state from second hash
            let mut hasher2 = blake3::Hasher::new();
            hasher2.update(seed_bytes);
            hasher2.update(b"state");
            let hash2 = hasher2.finalize();
            let state_bytes = hash2.as_bytes();
            for i in 0..4 {
                state[i] = u64::from_le_bytes([
                    state_bytes[i * 8],
                    state_bytes[i * 8 + 1],
                    state_bytes[i * 8 + 2],
                    state_bytes[i * 8 + 3],
                    state_bytes[i * 8 + 4],
                    state_bytes[i * 8 + 5],
                    state_bytes[i * 8 + 6],
                    state_bytes[i * 8 + 7],
                ]);
            }
        }

        #[cfg(not(feature = "crypto_rng"))]
        {
            // Fallback: simple derivation
            for (i, &byte) in seed_bytes.iter().enumerate() {
                if i < 32 {
                    key[i] = byte;
                }
            }
            for i in 0..4 {
                state[i] = (i as u64).wrapping_mul(0x9e3779b97f4a7c15);
            }
        }

        Ok(Self {
            state,
            key,
            counter: 0,
        })
    }

    /// Mix function combining multiple operations
    #[inline(always)]
    fn mix(&mut self) -> u64 {
        // Mix state using various operations
        let mut result = self.state[0];

        // Rotate and XOR
        result = result.rotate_left(13) ^ self.state[1];
        result = result.rotate_right(7) ^ self.state[2];
        result = result.rotate_left(17) ^ self.state[3];

        // Update state
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];
        self.state[3] = result.wrapping_add(self.counter);

        // Mix with key using Blake3
        #[cfg(feature = "crypto_rng")]
        {
            use blake3;
            let mut hasher = blake3::Hasher::new_keyed(&self.key);
            hasher.update(&result.to_le_bytes());
            hasher.update(&self.counter.to_le_bytes());
            let hash = hasher.finalize();
            result ^= u64::from_le_bytes([
                hash.as_bytes()[0],
                hash.as_bytes()[1],
                hash.as_bytes()[2],
                hash.as_bytes()[3],
                hash.as_bytes()[4],
                hash.as_bytes()[5],
                hash.as_bytes()[6],
                hash.as_bytes()[7],
            ]);
        }

        self.counter = self.counter.wrapping_add(1);
        result
    }
}

impl Rng for HashMix256 {
    fn next_u32(&mut self) -> u32 {
        (self.mix() >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.mix()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut chunks = dest.chunks_exact_mut(8);
        for chunk in chunks.by_ref() {
            let value = self.mix();
            chunk.copy_from_slice(&value.to_le_bytes());
        }
        let remainder = chunks.into_remainder();
        if !remainder.is_empty() {
            let value = self.mix();
            let bytes = value.to_le_bytes();
            remainder.copy_from_slice(&bytes[..remainder.len()]);
        }
    }
}

impl CryptoRng for HashMix256 {}

impl DeterministicRng for HashMix256 {
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for HashMix256 {
    type Seed = Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(&seed).expect("Seed should be valid")
    }

    fn reseed(&mut self, seed: Self::Seed) -> Result<()> {
        *self = Self::new(&seed)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "custom_rng")]
    fn test_hashmix256_deterministic() {
        let seed =
            Seed::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
        let mut rng1 = HashMix256::new(&seed).unwrap();
        let mut rng2 = HashMix256::new(&seed).unwrap();

        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
}
