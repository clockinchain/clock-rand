//! Distribution traits and implementations

pub mod uniform;

pub use uniform::Uniform;

/// Trait for generating random values from a distribution
pub trait Distribution<T> {
    /// Generate a random value from this distribution
    fn sample<R: crate::traits::Rng>(&self, rng: &mut R) -> T;
}
