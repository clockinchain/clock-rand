//! Core traits for RNG implementations

use crate::error::Result;

/// Core trait for random number generators
///
/// This trait provides the basic interface for generating random values.
/// It's compatible with the `rand` crate's `RngCore` trait for interoperability.
pub trait Rng {
    /// Generate the next `u32` random value
    fn next_u32(&mut self) -> u32;

    /// Generate the next `u64` random value
    fn next_u64(&mut self) -> u64;

    /// Fill a byte buffer with random data
    fn fill_bytes(&mut self, dest: &mut [u8]);

    /// Try to fill a byte buffer with random data (may fail)
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<()> {
        self.fill_bytes(dest);
        Ok(())
    }
}

/// Marker trait for cryptographically secure random number generators
///
/// Types implementing this trait are suitable for cryptographic operations
/// such as key generation, nonce generation, and signature operations.
pub trait CryptoRng: Rng {
    // Marker trait - no additional methods required
    // The security guarantee is provided by the implementation
}

/// Trait for deterministic random number generators
///
/// RNGs implementing this trait produce reproducible sequences when seeded
/// with the same seed value.
pub trait DeterministicRng: Rng {
    /// Check if the RNG is in a deterministic state
    fn is_deterministic(&self) -> bool;
}

/// Trait for RNGs that can be seeded
pub trait SeedableRng: Rng {
    /// Seed type for this RNG
    type Seed: AsRef<[u8]> + Clone;

    /// Create a new RNG from a seed
    fn from_seed(seed: Self::Seed) -> Self;

    /// Reseed the RNG with a new seed
    fn reseed(&mut self, seed: Self::Seed) -> Result<()>;
}

/// Compatibility trait with `rand` crate's `RngCore`
///
/// This allows clock-rand RNGs to be used with the `rand` crate ecosystem.
pub trait RngCore: Rng {
    /// Generate the next `u32` random value
    fn next_u32(&mut self) -> u32;

    /// Generate the next `u64` random value
    fn next_u64(&mut self) -> u64;

    /// Fill a byte buffer with random data
    fn fill_bytes(&mut self, dest: &mut [u8]);

    /// Try to fill a byte buffer with random data (may fail)
    #[cfg(feature = "std")]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> core::result::Result<(), std::io::Error>;
}

/// Helper trait for generating random values of specific types
pub trait RngExt: Rng {
    /// Generate a random `u8`
    #[inline]
    fn gen_u8(&mut self) -> u8 {
        (self.next_u32() >> 24) as u8
    }

    /// Generate a random `u16`
    #[inline]
    fn gen_u16(&mut self) -> u16 {
        (self.next_u32() >> 16) as u16
    }

    /// Generate a random `u32`
    #[inline]
    fn gen_u32(&mut self) -> u32 {
        self.next_u32()
    }

    /// Generate a random `u64`
    #[inline]
    fn gen_u64(&mut self) -> u64 {
        self.next_u64()
    }

    /// Generate a random `f32` in [0, 1)
    fn gen_f32(&mut self) -> f32 {
        // Generate 24 random bits for mantissa
        let bits = (self.next_u32() >> 8) | 0x3f80_0000;
        f32::from_bits(bits) - 1.0
    }

    /// Generate a random `f64` in [0, 1)
    fn gen_f64(&mut self) -> f64 {
        // Generate 53 random bits for mantissa
        let bits = (self.next_u64() >> 11) | 0x3ff0_0000_0000_0000;
        f64::from_bits(bits) - 1.0
    }
}

impl<T: Rng> RngExt for T {}

// Blanket implementation for RngCore compatibility
impl<T: Rng> RngCore for T {
    fn next_u32(&mut self) -> u32 {
        Rng::next_u32(self)
    }

    fn next_u64(&mut self) -> u64 {
        Rng::next_u64(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        Rng::fill_bytes(self, dest)
    }

    #[cfg(feature = "std")]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> core::result::Result<(), std::io::Error> {
        Rng::try_fill_bytes(self, dest).map_err(std::io::Error::other)
    }
}
