//! PCG64 implementation
//!
//! PCG (Permuted Congruential Generator) is a family of fast PRNGs
//! with excellent statistical properties. PCG64 uses 64-bit arithmetic.

use crate::error::Result;
use crate::fast::splitmix64::SplitMix64;
use crate::seed::Seed;
use crate::traits::{DeterministicRng, Rng, SeedableRng};

/// PCG64 PRNG
///
/// A fast, high-quality PRNG with 128-bit state. Excellent statistical
/// properties and small code footprint. Suitable for simulations and
/// non-cryptographic randomness.
#[derive(Debug, Clone)]
pub struct Pcg64 {
    state: u64,
    inc: u64,
}

impl Pcg64 {
    /// Create a new PCG64 from a seed
    pub fn new(seed: u64) -> Self {
        // Default increment (must be odd)
        let mut splitmix = SplitMix64::new(seed);
        let state = splitmix.next_u64();
        let inc = (splitmix.next_u64() << 1) | 1; // Ensure odd
        Self { state, inc }
    }

    /// Create from a Seed object
    pub fn from_seed_obj(seed: &Seed) -> Result<Self> {
        let seed_bytes = seed.as_ref();
        if seed_bytes.len() < 8 {
            // Expand seed if needed
            let mut expanded = seed_bytes.to_vec();
            while expanded.len() < 8 {
                expanded.extend_from_slice(seed_bytes);
            }
            let seed_value = u64::from_le_bytes([
                expanded[0],
                expanded[1],
                expanded[2],
                expanded[3],
                expanded[4],
                expanded[5],
                expanded[6],
                expanded[7],
            ]);
            Ok(Self::new(seed_value))
        } else {
            let seed_value = u64::from_le_bytes([
                seed_bytes[0],
                seed_bytes[1],
                seed_bytes[2],
                seed_bytes[3],
                seed_bytes[4],
                seed_bytes[5],
                seed_bytes[6],
                seed_bytes[7],
            ]);
            Ok(Self::new(seed_value))
        }
    }

    /// Create with custom state and increment
    pub fn from_state(state: u64, inc: u64) -> Self {
        // Ensure increment is odd
        let inc = inc | 1;
        Self { state, inc }
    }

    /// Generate the next u32 value
    #[inline(always)]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Generate the next u64 value
    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        let oldstate = self.state;

        // Advance state: state = state * 6364136223846793005 + inc
        self.state = oldstate
            .wrapping_mul(6364136223846793005u64)
            .wrapping_add(self.inc);

        // Output function: XSH-RR (xorshift high bits, rotate right)
        let xorshifted = ((oldstate >> 18) ^ oldstate) >> 27;
        let rot = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
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

    /// Save the current state
    pub fn save_state(&self) -> [u64; 2] {
        [self.state, self.inc]
    }

    /// Restore state from saved state
    pub fn restore_state(&mut self, state: [u64; 2]) {
        self.state = state[0];
        self.inc = state[1] | 1; // Ensure odd
    }
}

impl Rng for Pcg64 {
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

impl DeterministicRng for Pcg64 {
    #[inline]
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for Pcg64 {
    type Seed = Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self::from_seed_obj(&seed).expect("Seed should be valid")
    }

    fn reseed(&mut self, seed: Self::Seed) -> Result<()> {
        *self = Self::from_seed_obj(&seed)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcg64_deterministic() {
        let mut rng1 = Pcg64::new(12345);
        let mut rng2 = Pcg64::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_pcg64_save_restore() {
        let mut rng = Pcg64::new(42);
        let _ = rng.next_u64();
        let state = rng.save_state();

        let mut rng2 = Pcg64::new(0);
        rng2.restore_state(state);

        assert_eq!(rng.next_u64(), rng2.next_u64());
    }

    #[test]
    fn test_pcg64_increment_odd() {
        let rng = Pcg64::from_state(123, 456);
        // Increment should be made odd
        assert_eq!(rng.inc & 1, 1);
    }
}
