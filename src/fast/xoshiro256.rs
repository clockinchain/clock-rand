//! Xoshiro256+ implementation
//!
//! Xoshiro256+ is a fast, high-quality PRNG suitable for simulations
//! and non-cryptographic randomness. It's the successor to xoroshiro128+.

use crate::error::Result;
use crate::fast::splitmix64::SplitMix64;
use crate::seed::Seed;
use crate::traits::{DeterministicRng, Rng, SeedableRng};

/// Xoshiro256+ PRNG
///
/// A fast, high-quality PRNG with 256-bit state. Suitable for simulations,
/// games, and other non-cryptographic uses. Not suitable for security-critical
/// applications.
#[derive(Debug, Clone)]
pub struct Xoshiro256Plus {
    state: [u64; 4],
}

impl Xoshiro256Plus {
    /// Create a new Xoshiro256+ from a seed
    pub fn new(seed: u64) -> Self {
        let mut splitmix = SplitMix64::new(seed);
        Self {
            state: splitmix.seed_xoshiro256(),
        }
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

    /// Generate the next u64 value
    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        let result = self.state[0].wrapping_add(self.state[3]);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
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

    /// Jump function for generating non-overlapping sequences
    #[inline]
    pub fn jump(&mut self) {
        const JUMP: [u64; 4] = [
            0x180ec6d33cfd0aba,
            0xd5a61266f0c9392c,
            0xa9582618e03fc9aa,
            0x39abdc4529b1661c,
        ];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for j in &JUMP {
            for b in 0..64 {
                if (j & (1u64 << b)) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                self.next_u64();
            }
        }

        self.state[0] = s0;
        self.state[1] = s1;
        self.state[2] = s2;
        self.state[3] = s3;
    }

    /// Long jump function for generating even more distant sequences
    #[inline]
    pub fn long_jump(&mut self) {
        const LONG_JUMP: [u64; 4] = [
            0x76e15d3efefdcbbf,
            0xc5004e441c522fb3,
            0x77710069854ee241,
            0x39109bb02acbe635,
        ];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for j in &LONG_JUMP {
            for b in 0..64 {
                if (j & (1u64 << b)) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                self.next_u64();
            }
        }

        self.state[0] = s0;
        self.state[1] = s1;
        self.state[2] = s2;
        self.state[3] = s3;
    }

    /// Save the current state
    pub fn save_state(&self) -> [u64; 4] {
        self.state
    }

    /// Restore state from saved state
    pub fn restore_state(&mut self, state: [u64; 4]) {
        self.state = state;
    }
}

impl Rng for Xoshiro256Plus {
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

impl DeterministicRng for Xoshiro256Plus {
    #[inline]
    fn is_deterministic(&self) -> bool {
        true
    }
}

impl SeedableRng for Xoshiro256Plus {
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
    fn test_xoshiro256_deterministic() {
        let mut rng1 = Xoshiro256Plus::new(12345);
        let mut rng2 = Xoshiro256Plus::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_xoshiro256_jump() {
        let mut rng1 = Xoshiro256Plus::new(12345);
        let mut rng2 = Xoshiro256Plus::new(12345);

        // Generate some values
        for _ in 0..10 {
            rng1.next_u64();
        }

        // Jump should produce different sequence
        rng2.jump();
        assert_ne!(rng1.next_u64(), rng2.next_u64());
    }

    #[test]
    fn test_xoshiro256_save_restore() {
        let mut rng = Xoshiro256Plus::new(42);
        let _ = rng.next_u64();
        let state = rng.save_state();

        let mut rng2 = Xoshiro256Plus::new(0);
        rng2.restore_state(state);

        assert_eq!(rng.next_u64(), rng2.next_u64());
    }
}
