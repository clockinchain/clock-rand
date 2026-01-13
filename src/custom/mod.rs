//! Custom blockchain-aware RNGs
//!
//! This module contains RNGs specifically designed for blockchain applications,
//! including fork detection and blockchain state seeding.

#[cfg(feature = "custom_rng")]
pub mod chainseed_x;
#[cfg(feature = "custom_rng")]
pub mod entrocrypt;
#[cfg(feature = "custom_rng")]
pub mod hashmix256;

#[cfg(feature = "custom_rng")]
pub use chainseed_x::ChainSeedX;
#[cfg(feature = "custom_rng")]
pub use entrocrypt::EntroCrypt;
#[cfg(feature = "custom_rng")]
pub use hashmix256::HashMix256;
