//! SplitMix64 seeding algorithm
//!
//! SplitMix64 is a fast, high-quality seeding algorithm used to initialize
//! other PRNGs from a single seed value.

use crate::traits::Rng;

/// SplitMix64 PRNG for seeding other generators
///
/// This is an ultra-fast PRNG specifically designed for seeding other PRNGs.
/// It's not suitable for cryptographic use but provides excellent quality
/// for initialization purposes.
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Create a new SplitMix64 from a seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generate the next u64 value
    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    /// Generate the next u32 value
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Fill a buffer with random bytes
    #[inline]
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut chunks = dest.chunks_exact_mut(8);
        for chunk in chunks.by_ref() {
            let value = self.next_u64();
            chunk.copy_from_slice(&value.to_le_bytes());
        }
        let remainder = chunks.into_remainder();
        if !remainder.is_empty() {
            let value = self.next_u64();
            let bytes = value.to_le_bytes();
            remainder.copy_from_slice(&bytes[..remainder.len()]);
        }
    }

    /// Generate a seed array for Xoshiro256+
    pub fn seed_xoshiro256(&mut self) -> [u64; 4] {
        [
            self.next_u64(),
            self.next_u64(),
            self.next_u64(),
            self.next_u64(),
        ]
    }

    /// Generate a seed array for PCG64
    pub fn seed_pcg64(&mut self) -> [u64; 2] {
        [self.next_u64(), self.next_u64()]
    }
}

impl Rng for SplitMix64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.fill_bytes(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitmix64_deterministic() {
        let mut rng1 = SplitMix64::new(12345);
        let mut rng2 = SplitMix64::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_splitmix64_fill_bytes() {
        let mut rng = SplitMix64::new(42);
        let mut buf = [0u8; 32];
        rng.fill_bytes(&mut buf);

        // Should not be all zeros
        assert!(!buf.iter().all(|&b| b == 0));
    }
}
