//! EntroCrypt: Hybrid cryptographically secure RNG
//!
//! Combines ChaCha20 stream cipher with Blake3 keyed hash for maximum
//! security and performance. Thread-safe by design.

use crate::crypto::blake3_drbg::Blake3Drbg;
use crate::crypto::chacha20::ChaCha20Rng;
use crate::error::Result;
use crate::seed::Seed;
use crate::traits::{CryptoRng, DeterministicRng, Rng, SeedableRng};

#[cfg(feature = "security")]
use zeroize::Zeroize;

/// Builder for EntroCrypt RNG
pub struct EntroCryptBuilder {
    chain_entropy: Option<Vec<u8>>,
    external_entropy: Option<Vec<u8>>,
    reseed_interval: Option<u64>,
}

impl EntroCryptBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            chain_entropy: None,
            external_entropy: None,
            reseed_interval: None,
        }
    }

    /// Add chain entropy (from blockchain state)
    pub fn with_chain_entropy(mut self, entropy: &[u8]) -> Self {
        self.chain_entropy = Some(entropy.to_vec());
        self
    }

    /// Add external entropy source (e.g., OS RNG, Intel RDRAND)
    pub fn with_external_entropy(mut self, entropy: &[u8]) -> Self {
        self.external_entropy = Some(entropy.to_vec());
        self
    }

    /// Set reseed interval (number of bytes before reseeding)
    pub fn with_reseed_interval(mut self, interval: u64) -> Self {
        self.reseed_interval = Some(interval);
        self
    }

    /// Build the EntroCrypt RNG
    pub fn build(self) -> Result<EntroCrypt> {
        // Combine entropy sources
        let mut sources = Vec::new();

        if let Some(chain) = &self.chain_entropy {
            sources.push(chain.as_slice());
        }

        if let Some(external) = &self.external_entropy {
            sources.push(external.as_slice());
        }

        if sources.is_empty() {
            return Err(
                crate::error::SeedError::ValidationFailed("No entropy sources provided").into(),
            );
        }

        let seed = Seed::from_combined(&sources)?;

        Ok(EntroCrypt {
            chacha20: ChaCha20Rng::new(&seed)?,
            blake3: Blake3Drbg::new(&seed)?,
            bytes_generated: 0,
            reseed_interval: self.reseed_interval.unwrap_or(1_000_000_000), // 1GB default
        })
    }
}

impl Default for EntroCryptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// EntroCrypt: Hybrid cryptographically secure RNG
///
/// Combines ChaCha20 stream cipher with Blake3 keyed hash for maximum
/// security. Thread-safe and suitable for high-throughput applications.
#[derive(Clone)]
pub struct EntroCrypt {
    chacha20: ChaCha20Rng,
    blake3: Blake3Drbg,
    bytes_generated: u64,
    reseed_interval: u64,
}

#[cfg(feature = "security")]
impl Zeroize for EntroCrypt {
    fn zeroize(&mut self) {
        self.chacha20.zeroize();
        self.blake3.zeroize();
        self.bytes_generated = 0;
    }
}

impl EntroCrypt {
    /// Create a new builder
    pub fn builder() -> EntroCryptBuilder {
        EntroCryptBuilder::new()
    }

    /// Mix in additional chain entropy
    pub fn mix_chain_entropy(&mut self, entropy: &[u8]) -> Result<()> {
        let seed = Seed::from_combined(&[entropy])?;
        self.chacha20.reseed(seed.clone())?;
        self.blake3.reseed(seed)?;
        Ok(())
    }

    /// Check if reseeding is needed
    fn check_reseed(&mut self) -> Result<()> {
        if self.bytes_generated >= self.reseed_interval {
            // Reseed both RNGs
            let mut new_seed_bytes = vec![0u8; 32];
            self.chacha20.fill_bytes(&mut new_seed_bytes);
            let seed = Seed::from_bytes(new_seed_bytes)?;

            self.chacha20.reseed(seed.clone())?;
            self.blake3.reseed(seed)?;
            self.bytes_generated = 0;
        }
        Ok(())
    }
}

impl Rng for EntroCrypt {
    fn next_u32(&mut self) -> u32 {
        // XOR output from both RNGs
        let result = self.chacha20.next_u32() ^ self.blake3.next_u32();
        self.bytes_generated += 4;
        let _ = self.check_reseed();
        result
    }

    fn next_u64(&mut self) -> u64 {
        // XOR output from both RNGs
        let result = self.chacha20.next_u64() ^ self.blake3.next_u64();
        self.bytes_generated += 8;
        let _ = self.check_reseed();
        result
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // Use ChaCha20 for primary output, XOR with Blake3
        let mut blake3_bytes = vec![0u8; dest.len()];
        self.chacha20.fill_bytes(dest);
        self.blake3.fill_bytes(&mut blake3_bytes);

        for (d, b) in dest.iter_mut().zip(blake3_bytes.iter()) {
            *d ^= *b;
        }

        self.bytes_generated += dest.len() as u64;
        let _ = self.check_reseed();
    }
}

impl CryptoRng for EntroCrypt {}

impl DeterministicRng for EntroCrypt {
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for EntroCrypt {
    type Seed = Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self::builder()
            .with_chain_entropy(seed.as_ref())
            .build()
            .expect("Seed should be valid")
    }

    fn reseed(&mut self, seed: Self::Seed) -> Result<()> {
        self.chacha20.reseed(seed.clone())?;
        self.blake3.reseed(seed)?;
        self.bytes_generated = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "custom_rng")]
    fn test_entrocrypt_builder() {
        let entropy = b"test entropy";
        let rng = EntroCrypt::builder()
            .with_chain_entropy(entropy)
            .build()
            .unwrap();

        assert_eq!(rng.reseed_interval, 1_000_000_000);
    }

    #[test]
    #[cfg(feature = "custom_rng")]
    fn test_entrocrypt_deterministic() {
        let entropy = b"deterministic test";
        let mut rng1 = EntroCrypt::builder()
            .with_chain_entropy(entropy)
            .build()
            .unwrap();

        let mut rng2 = EntroCrypt::builder()
            .with_chain_entropy(entropy)
            .build()
            .unwrap();

        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
}
