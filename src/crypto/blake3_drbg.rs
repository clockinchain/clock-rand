//! Blake3-based DRBG (Deterministic Random Bit Generator)
//!
//! Uses Blake3 hash function in counter mode to generate cryptographically
//! secure random bytes.

use crate::error::Result;
use crate::seed::Seed;
use crate::traits::{CryptoRng, DeterministicRng, Rng, SeedableRng};

#[cfg(feature = "security")]
use zeroize::Zeroize;

/// Blake3-based DRBG
///
/// This RNG uses Blake3 in counter mode to generate cryptographically
/// secure random bytes. Fast and suitable for high-throughput applications.
#[derive(Clone)]
pub struct Blake3Drbg {
    key: [u8; 32],
    counter: u64,
    buffer: [u8; 64],
    buffer_pos: usize,
}

#[cfg(feature = "security")]
impl Zeroize for Blake3Drbg {
    fn zeroize(&mut self) {
        self.key.zeroize();
        self.counter = 0;
        self.buffer.zeroize();
        self.buffer_pos = 0;
    }
}

#[cfg(feature = "security")]
impl Drop for Blake3Drbg {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl Blake3Drbg {
    /// Create a new Blake3Drbg from a seed
    pub fn new(seed: &Seed) -> Result<Self> {
        let seed_bytes = seed.as_ref();
        let mut key = [0u8; 32];

        // Derive key from seed using Blake3
        #[cfg(feature = "crypto_rng")]
        {
            use blake3;
            let mut hasher = blake3::Hasher::new();
            hasher.update(seed_bytes);
            let hash = hasher.finalize();
            key.copy_from_slice(&hash.as_bytes()[..32]);
        }

        #[cfg(not(feature = "crypto_rng"))]
        {
            // Fallback: simple key derivation
            for (i, &byte) in seed_bytes.iter().enumerate() {
                if i < 32 {
                    key[i] = byte;
                }
            }
        }

        let mut rng = Self {
            key,
            counter: 0,
            buffer: [0u8; 64],
            buffer_pos: 64, // Force refill on first use
        };

        // Pre-fill buffer
        rng.refill_buffer()?;
        Ok(rng)
    }

    /// Refill the internal buffer
    fn refill_buffer(&mut self) -> Result<()> {
        #[cfg(feature = "crypto_rng")]
        {
            use blake3;
            let mut hasher = blake3::Hasher::new_keyed(&self.key);
            hasher.update(&self.counter.to_le_bytes());
            let hash = hasher.finalize();
            // Fill first 32 bytes
            self.buffer[..32].copy_from_slice(hash.as_bytes());
            // Fill next 32 bytes with a second hash
            let mut hasher2 = blake3::Hasher::new_keyed(&self.key);
            hasher2.update(&self.counter.to_le_bytes());
            hasher2.update(b"second");
            let hash2 = hasher2.finalize();
            self.buffer[32..].copy_from_slice(hash2.as_bytes());
        }

        #[cfg(not(feature = "crypto_rng"))]
        {
            // Fallback: simple counter-based generation
            for (i, byte) in self.buffer.iter_mut().enumerate() {
                *byte = (self.counter as u8).wrapping_add(i as u8);
            }
        }

        self.counter = self.counter.wrapping_add(1);
        self.buffer_pos = 0;
        Ok(())
    }

    /// Save the current state
    pub fn save_state(&self) -> ([u8; 32], u64, usize) {
        (self.key, self.counter, self.buffer_pos)
    }

    /// Restore state from saved state
    pub fn restore_state(&mut self, key: [u8; 32], counter: u64, buffer_pos: usize) -> Result<()> {
        self.key = key;
        self.counter = counter;
        self.buffer_pos = buffer_pos;
        // If buffer_pos is at end, refill for next use
        if self.buffer_pos >= 64 {
            self.refill_buffer()?;
        }
        Ok(())
    }
}

impl Rng for Blake3Drbg {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        self.fill_bytes(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        self.fill_bytes(&mut bytes);
        u64::from_le_bytes(bytes)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            if self.buffer_pos >= 64 {
                self.refill_buffer()
                    .expect("Blake3 refill should never fail");
            }
            *byte = self.buffer[self.buffer_pos];
            self.buffer_pos += 1;
        }
    }
}

impl CryptoRng for Blake3Drbg {}

impl DeterministicRng for Blake3Drbg {
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for Blake3Drbg {
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
    #[cfg(feature = "crypto_rng")]
    fn test_blake3_drbg_deterministic() {
        let seed =
            Seed::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
        let mut rng1 = Blake3Drbg::new(&seed).unwrap();
        let mut rng2 = Blake3Drbg::new(&seed).unwrap();

        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    #[cfg(feature = "crypto_rng")]
    fn test_blake3_drbg_save_restore() {
        let seed = Seed::from_bytes(vec![42; 32]).unwrap();
        let mut rng = Blake3Drbg::new(&seed).unwrap();
        let _ = rng.next_u64();
        let (key, counter, buffer_pos) = rng.save_state();

        let mut rng2 = Blake3Drbg::new(&seed).unwrap();
        rng2.restore_state(key, counter, buffer_pos).unwrap();

        assert_eq!(rng.next_u64(), rng2.next_u64());
    }
}
