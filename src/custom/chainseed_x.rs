//! ChainSeed-X: Hybrid blockchain-aware RNG
//!
//! Combines Blake3-DRBG for initial mixing with PCG64 for speed.
//! Includes fork detection and automatic reseeding.

use crate::crypto::blake3_drbg::Blake3Drbg;
use crate::error::Result;
use crate::fast::pcg64::Pcg64;
use crate::seed::Seed;
use crate::traits::{CryptoRng, DeterministicRng, Rng, SeedableRng};

#[cfg(feature = "security")]
use zeroize::Zeroize;

/// Builder for ChainSeed-X RNG
pub struct ChainSeedXBuilder {
    block_hashes: Vec<[u8; 32]>,
    timestamp: Option<u64>,
    vrf_output: Option<Vec<u8>>,
    fork_detection_enabled: bool,
    fork_history_size: usize,
}

impl ChainSeedXBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            block_hashes: Vec::new(),
            timestamp: None,
            vrf_output: None,
            fork_detection_enabled: true,
            fork_history_size: 3,
        }
    }

    /// Add a block hash (up to 3 recent blocks)
    pub fn with_block_hash(mut self, hash: [u8; 32]) -> Self {
        if self.block_hashes.len() < 3 {
            self.block_hashes.push(hash);
        }
        self
    }

    /// Add multiple block hashes
    pub fn with_block_hashes(mut self, hashes: &[[u8; 32]]) -> Self {
        for hash in hashes.iter().take(3) {
            if self.block_hashes.len() < 3 {
                self.block_hashes.push(*hash);
            }
        }
        self
    }

    /// Set the timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Set the VRF output
    pub fn with_vrf(mut self, vrf: &[u8]) -> Self {
        self.vrf_output = Some(vrf.to_vec());
        self
    }

    /// Enable or disable fork detection
    pub fn with_fork_detection(mut self, enabled: bool) -> Self {
        self.fork_detection_enabled = enabled;
        self
    }

    /// Set the fork history size
    pub fn with_fork_history_size(mut self, size: usize) -> Self {
        self.fork_history_size = size;
        self
    }

    /// Build the ChainSeed-X RNG
    pub fn build(self) -> Result<ChainSeedX> {
        // Combine all seed sources
        let mut sources = Vec::new();

        for hash in &self.block_hashes {
            sources.push(hash.as_slice());
        }

        let ts_bytes = self.timestamp.map(|ts| ts.to_le_bytes());

        if let Some(ref bytes) = ts_bytes {
            sources.push(bytes);
        }

        if let Some(vrf) = &self.vrf_output {
            sources.push(vrf.as_slice());
        }

        if sources.is_empty() {
            return Err(
                crate::error::SeedError::ValidationFailed("No seed sources provided").into(),
            );
        }

        let seed = Seed::from_combined(&sources)?;

        Ok(ChainSeedX {
            blake3_drbg: Blake3Drbg::new(&seed)?,
            pcg: Pcg64::from_seed_obj(&seed)?,
            block_hashes: self.block_hashes,
            fork_detection_enabled: self.fork_detection_enabled,
            fork_history_size: self.fork_history_size,
        })
    }
}

impl Default for ChainSeedXBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// ChainSeed-X: Hybrid blockchain-aware RNG
///
/// Combines Blake3-DRBG for cryptographic quality with PCG64 for speed.
/// Automatically detects blockchain forks and reseeds when necessary.
#[derive(Clone)]
pub struct ChainSeedX {
    blake3_drbg: Blake3Drbg,
    pcg: Pcg64,
    block_hashes: Vec<[u8; 32]>,
    fork_detection_enabled: bool,
    fork_history_size: usize,
}

#[cfg(feature = "security")]
impl Zeroize for ChainSeedX {
    fn zeroize(&mut self) {
        self.blake3_drbg.zeroize();
        // PCG doesn't need zeroization (non-crypto)
        self.block_hashes.clear();
    }
}

impl ChainSeedX {
    /// Create a new builder
    pub fn builder() -> ChainSeedXBuilder {
        ChainSeedXBuilder::new()
    }

    /// Check for fork and reseed if necessary
    pub fn check_fork(&mut self, current_hash: &[u8; 32]) -> Result<bool> {
        if !self.fork_detection_enabled {
            return Ok(false);
        }

        // Check if current hash matches expected
        if let Some(last_hash) = self.block_hashes.last()
            && last_hash == current_hash
        {
            return Ok(false); // No fork
        }

        // Fork detected - reseed
        self.block_hashes.push(*current_hash);

        // Keep only recent history
        if self.block_hashes.len() > self.fork_history_size {
            self.block_hashes.remove(0);
        }

        // Reseed both RNGs
        let seed = Seed::from_block_hash(current_hash)?;
        self.blake3_drbg.reseed(seed.clone())?;
        self.pcg.reseed(seed)?;

        Ok(true) // Fork detected and handled
    }

    /// Update with new block hash (for continuous operation)
    pub fn update_block(&mut self, block_hash: &[u8; 32], timestamp: u64) -> Result<()> {
        let fork_detected = self.check_fork(block_hash)?;

        if !fork_detected {
            // Normal block progression - mix in new entropy
            let timestamp_bytes = timestamp.to_le_bytes();
            let mut sources = vec![block_hash.as_slice(), &timestamp_bytes];
            if let Some(vrf) = self.block_hashes.last() {
                sources.push(vrf.as_slice());
            }
            let seed = Seed::from_combined(&sources)?;

            // Light reseeding (only PCG for speed)
            self.pcg.reseed(seed)?;
        }

        Ok(())
    }
}

impl Rng for ChainSeedX {
    fn next_u32(&mut self) -> u32 {
        // XOR output from both RNGs for combined quality
        self.blake3_drbg.next_u32() ^ self.pcg.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        // XOR output from both RNGs
        self.blake3_drbg.next_u64() ^ self.pcg.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // Use Blake3 for crypto quality, XOR with PCG for speed
        let mut pcg_bytes = vec![0u8; dest.len()];
        self.blake3_drbg.fill_bytes(dest);
        self.pcg.fill_bytes(&mut pcg_bytes);

        for (d, p) in dest.iter_mut().zip(pcg_bytes.iter()) {
            *d ^= *p;
        }
    }
}

impl CryptoRng for ChainSeedX {}

impl DeterministicRng for ChainSeedX {
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for ChainSeedX {
    type Seed = Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        let seed_bytes = seed.as_ref();
        let mut hash = [0u8; 32];
        if seed_bytes.len() >= 32 {
            hash.copy_from_slice(&seed_bytes[..32]);
        } else {
            // Expand seed to 32 bytes
            let mut expanded = seed_bytes.to_vec();
            while expanded.len() < 32 {
                expanded.extend_from_slice(seed_bytes);
            }
            hash.copy_from_slice(&expanded[..32]);
        }

        Self::builder()
            .with_block_hash(hash)
            .build()
            .expect("Seed should be valid")
    }

    fn reseed(&mut self, seed: Self::Seed) -> Result<()> {
        let hash = if seed.len() >= 32 {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&seed.as_ref()[..32]);
            hash
        } else {
            // Expand seed to 32 bytes
            let mut expanded = seed.as_ref().to_vec();
            while expanded.len() < 32 {
                expanded.extend_from_slice(seed.as_ref());
            }
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&expanded[..32]);
            hash
        };

        self.blake3_drbg.reseed(seed.clone())?;
        self.pcg.reseed(seed)?;
        self.block_hashes = vec![hash];
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "custom_rng")]
    fn test_chainseed_x_builder() {
        let hash = [0x42u8; 32];
        let rng = ChainSeedX::builder()
            .with_block_hash(hash)
            .with_timestamp(12345)
            .build()
            .unwrap();

        assert!(!rng.block_hashes.is_empty());
    }

    #[test]
    #[cfg(feature = "custom_rng")]
    fn test_chainseed_x_fork_detection() {
        let hash1 = [0x01u8; 32];
        let hash2 = [0x02u8; 32];

        let mut rng = ChainSeedX::builder()
            .with_block_hash(hash1)
            .with_fork_detection(true)
            .build()
            .unwrap();

        // Different hash should trigger fork
        let fork_detected = rng.check_fork(&hash2).unwrap();
        assert!(fork_detected);
    }
}
