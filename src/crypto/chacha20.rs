//! ChaCha20-based cryptographically secure RNG
//!
//! Uses the ChaCha20 stream cipher as a DRBG (Deterministic Random Bit Generator).

use crate::error::Result;
use crate::seed::Seed;
use crate::traits::{CryptoRng, DeterministicRng, Rng, SeedableRng};

#[cfg(feature = "security")]
use zeroize::Zeroize;

/// ChaCha20-based cryptographically secure RNG
///
/// This RNG uses ChaCha20 in counter mode to generate cryptographically
/// secure random bytes. Suitable for key generation, nonces, and other
/// security-critical operations.
#[derive(Clone)]
pub struct ChaCha20Rng {
    key: [u8; 32],
    nonce: [u8; 12],
    counter: u64,
    buffer: [u8; 64],
    buffer_pos: usize,
}

#[cfg(feature = "security")]
impl Zeroize for ChaCha20Rng {
    fn zeroize(&mut self) {
        self.key.zeroize();
        self.nonce.zeroize();
        self.counter = 0;
        self.buffer.zeroize();
        self.buffer_pos = 0;
    }
}

#[cfg(feature = "security")]
impl Drop for ChaCha20Rng {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl ChaCha20Rng {
    /// Create a new ChaCha20Rng from a seed
    pub fn new(seed: &Seed) -> Result<Self> {
        let seed_bytes = seed.as_ref();
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 12];

        // Derive key and nonce from seed using Blake3
        #[cfg(feature = "crypto_rng")]
        {
            use blake3;
            let mut hasher = blake3::Hasher::new();
            hasher.update(seed_bytes);
            let hash = hasher.finalize();
            key.copy_from_slice(&hash.as_bytes()[..32]);

            // Derive nonce from second hash
            let mut hasher2 = blake3::Hasher::new();
            hasher2.update(seed_bytes);
            hasher2.update(b"nonce");
            let hash2 = hasher2.finalize();
            nonce.copy_from_slice(&hash2.as_bytes()[..12]);
        }

        #[cfg(not(feature = "crypto_rng"))]
        {
            // Fallback: simple key derivation (not cryptographically secure)
            for (i, &byte) in seed_bytes.iter().enumerate() {
                if i < 32 {
                    key[i] = byte;
                } else if i < 44 {
                    nonce[i - 32] = byte;
                }
            }
        }

        let mut rng = Self {
            key,
            nonce,
            counter: 0,
            buffer: [0u8; 64],
            buffer_pos: 64, // Force refill on first use
        };

        // Pre-fill buffer
        rng.refill_buffer()?;
        Ok(rng)
    }

    /// Create from OS entropy (requires std)
    #[cfg(feature = "std")]
    pub fn from_os_entropy() -> Result<Self> {
        use crate::error::EntropyError;
        let mut seed_bytes = vec![0u8; 32];

        use std::fs::File;
        use std::io::Read;

        // Try to read from /dev/urandom (Unix) or use getrandom
        #[cfg(unix)]
        {
            let mut file = File::open("/dev/urandom")
                .map_err(|_| EntropyError::ReadFailed("Failed to open /dev/urandom"))?;
            file.read_exact(&mut seed_bytes)
                .map_err(|_| EntropyError::ReadFailed("Failed to read from /dev/urandom"))?;
        }

        #[cfg(not(unix))]
        {
            // Fallback: use system RNG if available
            return Err(EntropyError::SourceUnavailable.into());
        }

        let seed = Seed::from_bytes(seed_bytes)?;
        Self::new(&seed)
    }

    /// Refill the internal buffer
    fn refill_buffer(&mut self) -> Result<()> {
        #[cfg(feature = "crypto_rng")]
        {
            use chacha20::ChaCha20;
            use chacha20::cipher::{KeyIvInit, StreamCipher};

            let key = chacha20::Key::from_slice(&self.key);

            // ChaCha20 uses the first 4 bytes of nonce as counter in IETF variant
            let mut nonce_bytes = [0u8; 12];
            nonce_bytes[0..4].copy_from_slice(&self.counter.to_le_bytes()[0..4]);
            nonce_bytes[4..12].copy_from_slice(&self.nonce[0..8]);

            let nonce = chacha20::Nonce::from_slice(&nonce_bytes);
            let mut cipher = ChaCha20::new(key, nonce);

            cipher.apply_keystream(&mut self.buffer);
        }

        #[cfg(not(feature = "crypto_rng"))]
        {
            // Fallback: simple counter-based generation (not secure)
            for (i, byte) in self.buffer.iter_mut().enumerate() {
                *byte = (self.counter as u8).wrapping_add(i as u8);
            }
        }

        self.counter = self.counter.wrapping_add(1);
        self.buffer_pos = 0;
        Ok(())
    }

    /// Save the current state
    pub fn save_state(&self) -> ([u8; 32], [u8; 12], u64, usize) {
        (self.key, self.nonce, self.counter, self.buffer_pos)
    }

    /// Restore state from saved state
    pub fn restore_state(
        &mut self,
        key: [u8; 32],
        nonce: [u8; 12],
        counter: u64,
        buffer_pos: usize,
    ) -> Result<()> {
        self.key = key;
        self.nonce = nonce;
        self.counter = counter;
        self.buffer_pos = buffer_pos;
        // If buffer_pos is at end, refill for next use
        if self.buffer_pos >= 64 {
            self.refill_buffer()?;
        }
        Ok(())
    }
}

impl Rng for ChaCha20Rng {
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
                    .expect("ChaCha20 refill should never fail");
            }
            *byte = self.buffer[self.buffer_pos];
            self.buffer_pos += 1;
        }
    }
}

impl CryptoRng for ChaCha20Rng {}

impl DeterministicRng for ChaCha20Rng {
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for ChaCha20Rng {
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
    fn test_chacha20_deterministic() {
        let seed =
            Seed::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
        let mut rng1 = ChaCha20Rng::new(&seed).unwrap();
        let mut rng2 = ChaCha20Rng::new(&seed).unwrap();

        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    #[cfg(feature = "crypto_rng")]
    fn test_chacha20_save_restore() {
        let seed = Seed::from_bytes(vec![42; 32]).unwrap();
        let mut rng = ChaCha20Rng::new(&seed).unwrap();
        let _ = rng.next_u64();
        let (key, nonce, counter, buffer_pos) = rng.save_state();

        let mut rng2 = ChaCha20Rng::new(&seed).unwrap();
        rng2.restore_state(key, nonce, counter, buffer_pos).unwrap();

        assert_eq!(rng.next_u64(), rng2.next_u64());
    }
}
