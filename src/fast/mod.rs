//! Fast deterministic RNGs
//!
//! This module contains high-performance PRNGs suitable for simulations
//! and non-cryptographic randomness. These RNGs are deterministic when
//! seeded and provide excellent statistical properties.

pub mod pcg64;
pub mod splitmix64;
pub mod xoshiro256;

pub use pcg64::Pcg64;
pub use splitmix64::SplitMix64;
pub use xoshiro256::Xoshiro256Plus;
