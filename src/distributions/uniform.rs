//! Uniform distribution

use crate::distributions::Distribution;
use crate::traits::{Rng, RngExt};

/// Uniform distribution over a range
#[derive(Debug, Clone, Copy)]
pub struct Uniform<T> {
    low: T,
    high: T,
}

impl Uniform<u32> {
    /// Create a uniform distribution over [low, high)
    pub fn new(low: u32, high: u32) -> Self {
        assert!(low < high, "low must be less than high");
        Self { low, high }
    }

    /// Create a uniform distribution over [low, high) - type-specific constructor
    pub fn new_u32(low: u32, high: u32) -> Self {
        Self::new(low, high)
    }

    /// Create a uniform distribution over [0, high)
    pub fn to(high: u32) -> Self {
        Self::new(0, high)
    }
}

impl Uniform<u64> {
    /// Create a uniform distribution over [low, high)
    pub fn new(low: u64, high: u64) -> Self {
        assert!(low < high, "low must be less than high");
        Self { low, high }
    }

    /// Create a uniform distribution over [low, high) - type-specific constructor
    pub fn new_u64(low: u64, high: u64) -> Self {
        Self::new(low, high)
    }

    /// Create a uniform distribution over [0, high)
    pub fn to(high: u64) -> Self {
        Self::new(0, high)
    }
}

impl Uniform<f32> {
    /// Create a uniform distribution over [low, high)
    pub fn new(low: f32, high: f32) -> Self {
        assert!(low < high, "low must be less than high");
        Self { low, high }
    }

    /// Create a uniform distribution over [low, high) - type-specific constructor
    pub fn new_f32(low: f32, high: f32) -> Self {
        Self::new(low, high)
    }

    /// Create a uniform distribution over [0.0, 1.0)
    pub fn unit() -> Self {
        Self::new(0.0, 1.0)
    }
}

impl Uniform<f64> {
    /// Create a uniform distribution over [low, high)
    pub fn new(low: f64, high: f64) -> Self {
        assert!(low < high, "low must be less than high");
        Self { low, high }
    }

    /// Create a uniform distribution over [low, high) - type-specific constructor
    pub fn new_f64(low: f64, high: f64) -> Self {
        Self::new(low, high)
    }

    /// Create a uniform distribution over [0.0, 1.0)
    pub fn unit() -> Self {
        Self::new(0.0, 1.0)
    }
}

impl Distribution<u32> for Uniform<u32> {
    fn sample<R: Rng>(&self, rng: &mut R) -> u32 {
        let range = self.high - self.low;
        let mut x = rng.next_u32();
        let mut m = (x as u64).wrapping_mul(range as u64);
        let mut threshold = (!range + 1) % range;

        while m < threshold as u64 {
            x = rng.next_u32();
            m = (x as u64).wrapping_mul(range as u64);
            threshold = (!range + 1) % range;
        }

        (m >> 32) as u32 + self.low
    }
}

impl Distribution<u64> for Uniform<u64> {
    fn sample<R: Rng>(&self, rng: &mut R) -> u64 {
        let range = self.high - self.low;
        if range == 0 {
            return self.low;
        }

        let mut x = rng.next_u64();
        let mut m = x.wrapping_mul(range);
        let threshold = (!range).wrapping_add(1) % range;

        while m < threshold {
            x = rng.next_u64();
            m = x.wrapping_mul(range);
        }

        (m % range) + self.low
    }
}

impl Distribution<f32> for Uniform<f32> {
    fn sample<R: Rng>(&self, rng: &mut R) -> f32 {
        let x = rng.gen_f32();
        x * (self.high - self.low) + self.low
    }
}

impl Distribution<f64> for Uniform<f64> {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        let x = rng.gen_f64();
        x * (self.high - self.low) + self.low
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fast::xoshiro256::Xoshiro256Plus;

    #[test]
    fn test_uniform_u32() {
        let mut rng = Xoshiro256Plus::new(42);
        let dist = Uniform::new_u32(10, 20);

        for _ in 0..100 {
            let x = dist.sample(&mut rng);
            assert!(x >= 10 && x < 20);
        }
    }

    #[test]
    fn test_uniform_f64() {
        let mut rng = Xoshiro256Plus::new(42);
        let dist = Uniform::<f64>::unit();

        for _ in 0..100 {
            let x = dist.sample(&mut rng);
            assert!(x >= 0.0 && x < 1.0);
        }
    }
}
