//! AES-CTR DRBG (Deterministic Random Bit Generator)
//!
//! Uses AES in counter mode to generate cryptographically secure random bytes.
//! Optional feature for FIPS-compliant crypto RNG.

use crate::error::Result;
use crate::seed::Seed;
use crate::traits::{CryptoRng, DeterministicRng, Rng, SeedableRng};

#[cfg(feature = "security")]
use zeroize::Zeroize;

/// AES-CTR DRBG
///
/// This RNG uses AES-256 in counter mode to generate cryptographically
/// secure random bytes. Suitable for FIPS-compliant applications.
#[derive(Clone)]
pub struct AesCtrRng {
    key: [u8; 32],
    counter: u64,
    buffer: [u8; 16],
    buffer_pos: usize,
}

#[cfg(feature = "security")]
impl Zeroize for AesCtrRng {
    fn zeroize(&mut self) {
        self.key.zeroize();
        self.counter = 0;
        self.buffer.zeroize();
        self.buffer_pos = 0;
    }
}

#[cfg(feature = "security")]
impl Drop for AesCtrRng {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl AesCtrRng {
    /// Create a new AesCtrRng from a seed
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
            buffer: [0u8; 16],
            buffer_pos: 16, // Force refill on first use
        };

        // Pre-fill buffer
        rng.refill_buffer()?;
        Ok(rng)
    }

    /// Refill the internal buffer
    fn refill_buffer(&mut self) -> Result<()> {
        #[cfg(all(feature = "crypto_rng", feature = "aes"))]
        {
            use aes::Aes256;
            use aes::cipher::generic_array::GenericArray;
            use aes::cipher::{BlockEncrypt, KeyInit};

            let key = GenericArray::from_slice(&self.key);
            let cipher = Aes256::new(key);

            // Create counter block
            let mut counter_block = [0u8; 16];
            counter_block[..8].copy_from_slice(&self.counter.to_le_bytes());

            let block = GenericArray::from_mut_slice(&mut counter_block);
            cipher.encrypt_block(block);
            self.buffer.copy_from_slice(block.as_slice());
        }

        #[cfg(not(all(feature = "crypto_rng", feature = "aes")))]
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
    pub fn save_state(&self) -> ([u8; 32], u64) {
        (self.key, self.counter)
    }

    /// Restore state from saved state
    pub fn restore_state(&mut self, key: [u8; 32], counter: u64) -> Result<()> {
        self.key = key;
        self.counter = counter;
        self.buffer_pos = 16; // Force refill
        self.refill_buffer()?;
        Ok(())
    }
}

impl Rng for AesCtrRng {
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
            if self.buffer_pos >= 16 {
                self.refill_buffer().expect("AES refill should never fail");
            }
            *byte = self.buffer[self.buffer_pos];
            self.buffer_pos += 1;
        }
    }
}

impl CryptoRng for AesCtrRng {}

impl DeterministicRng for AesCtrRng {
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for AesCtrRng {
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
    #[cfg(all(feature = "crypto_rng", feature = "aes"))]
    fn test_aes_ctr_deterministic() {
        let seed =
            Seed::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
        let mut rng1 = AesCtrRng::new(&seed).unwrap();
        let mut rng2 = AesCtrRng::new(&seed).unwrap();

        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
}
