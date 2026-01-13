//! Seed types and blockchain state seeding

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::error::{Result, SeedError};

/// Seed type for RNG initialization
///
/// Seeds can be created from various blockchain state sources
/// including block hashes, timestamps, and VRF outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seed {
    bytes: Vec<u8>,
}

impl Seed {
    /// Create a seed from raw bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        if bytes.is_empty() {
            return Err(SeedError::InvalidSize {
                expected: 1,
                got: 0,
            }
            .into());
        }

        // Check for all zeros (weak seed)
        if bytes.iter().all(|&b| b == 0) {
            return Err(SeedError::AllZeros.into());
        }

        Ok(Self { bytes })
    }

    /// Create a seed from a block hash (32 bytes)
    pub fn from_block_hash(hash: &[u8; 32]) -> Result<Self> {
        Self::from_bytes(hash.to_vec())
    }

    /// Create a seed from a block hash slice
    pub fn from_block_hash_slice(hash: &[u8]) -> Result<Self> {
        if hash.len() < 16 {
            return Err(SeedError::InvalidSize {
                expected: 16,
                got: hash.len(),
            }
            .into());
        }
        Self::from_bytes(hash.to_vec())
    }

    /// Create a seed from a timestamp
    pub fn from_timestamp(timestamp: u64) -> Result<Self> {
        let bytes = timestamp.to_le_bytes().to_vec();
        Self::from_bytes(bytes)
    }

    /// Create a seed from a VRF output
    pub fn from_vrf(vrf_output: &[u8]) -> Result<Self> {
        if vrf_output.is_empty() {
            return Err(SeedError::InvalidSize {
                expected: 1,
                got: 0,
            }
            .into());
        }
        Self::from_bytes(vrf_output.to_vec())
    }

    /// Combine multiple seed sources into one seed
    ///
    /// Uses Blake3 to hash all inputs together for uniform distribution
    pub fn from_combined(sources: &[&[u8]]) -> Result<Self> {
        if sources.is_empty() {
            return Err(SeedError::ValidationFailed("No seed sources provided").into());
        }

        #[cfg(feature = "crypto_rng")]
        {
            use blake3;
            let mut hasher = blake3::Hasher::new();
            for source in sources {
                hasher.update(source);
            }
            let hash = hasher.finalize();
            Self::from_bytes(hash.as_bytes().to_vec())
        }

        #[cfg(not(feature = "crypto_rng"))]
        {
            // Fallback: simple XOR mixing when crypto_rng feature is disabled
            let mut combined = Vec::new();
            let max_len = sources.iter().map(|s| s.len()).max().unwrap_or(0);

            if max_len == 0 {
                return Err(SeedError::ValidationFailed("All seed sources are empty").into());
            }

            combined.resize(max_len, 0u8);
            for source in sources {
                for (i, &byte) in source.iter().enumerate() {
                    if i < combined.len() {
                        combined[i] ^= byte;
                    }
                }
            }
            Self::from_bytes(combined)
        }
    }

    /// Create a seed from block hash, timestamp, and optional VRF
    pub fn from_blockchain_state(
        block_hash: &[u8; 32],
        timestamp: u64,
        vrf_output: Option<&[u8]>,
    ) -> Result<Self> {
        let timestamp_bytes = timestamp.to_le_bytes();
        let mut sources = Vec::new();
        sources.push(block_hash.as_slice());
        sources.push(&timestamp_bytes);
        if let Some(vrf) = vrf_output {
            sources.push(vrf);
        }
        Self::from_combined(&sources)
    }

    /// Get the seed as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get the seed length in bytes
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Check if the seed is empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Expand or truncate seed to a specific size
    pub fn resize(&mut self, size: usize) -> Result<()> {
        if size == 0 {
            return Err(SeedError::InvalidSize {
                expected: 1,
                got: 0,
            }
            .into());
        }

        if self.bytes.len() < size {
            // Expand by hashing the current seed
            #[cfg(feature = "crypto_rng")]
            {
                use blake3;
                let mut hasher = blake3::Hasher::new();
                hasher.update(&self.bytes);
                let mut counter = 0u64;
                while self.bytes.len() < size {
                    let mut hasher2 = hasher.clone();
                    hasher2.update(&counter.to_le_bytes());
                    let hash = hasher2.finalize();
                    self.bytes.extend_from_slice(hash.as_bytes());
                    counter += 1;
                }
            }

            #[cfg(not(feature = "crypto_rng"))]
            {
                // Simple expansion: repeat with XOR
                let original = self.bytes.clone();
                while self.bytes.len() < size {
                    let needed = size - self.bytes.len();
                    let to_copy = needed.min(original.len());
                    for &byte in original.iter().take(to_copy) {
                        self.bytes.push(byte ^ (self.bytes.len() as u8));
                    }
                }
            }
        }

        self.bytes.truncate(size);
        Ok(())
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl From<[u8; 32]> for Seed {
    fn from(bytes: [u8; 32]) -> Self {
        Self::from_bytes(bytes.to_vec()).expect("32-byte array is always valid")
    }
}

impl From<Vec<u8>> for Seed {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_bytes(bytes).expect("Seed validation should be done explicitly")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_from_block_hash() {
        let hash = [0x42u8; 32];
        let seed = Seed::from_block_hash(&hash).unwrap();
        assert_eq!(seed.as_bytes(), &hash);
    }

    #[test]
    fn test_seed_from_timestamp() {
        let timestamp = 1234567890u64;
        let seed = Seed::from_timestamp(timestamp).unwrap();
        assert_eq!(seed.as_bytes(), &timestamp.to_le_bytes());
    }

    #[test]
    fn test_seed_rejects_all_zeros() {
        let zeros = Vec::from([0u8; 32]);
        assert!(Seed::from_bytes(zeros).is_err());
    }

    #[test]
    fn test_seed_from_combined() {
        let source1 = b"source1";
        let source2 = b"source2";
        let seed = Seed::from_combined(&[source1, source2]).unwrap();
        assert!(!seed.is_empty());
    }
}
