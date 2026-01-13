//! Serialization support for RNG states
//!
//! Provides Serialize/Deserialize implementations for checkpointing
//! and state management.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Versioned state format for compatibility
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RngState {
    /// Version of the state format
    pub version: u32,
    /// RNG type identifier
    pub rng_type: String,
    /// Serialized state data
    pub state_data: Vec<u8>,
}

impl RngState {
    /// Current version of the state format
    pub const CURRENT_VERSION: u32 = 1;

    /// Create a new RngState
    pub fn new(rng_type: String, state_data: Vec<u8>) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            rng_type,
            state_data,
        }
    }

    /// Check if the state version is compatible
    pub fn is_compatible(&self) -> bool {
        self.version == Self::CURRENT_VERSION
    }
}

/// Trait for RNGs that can save and restore their state
pub trait StatefulRng {
    /// Save the current state
    fn save_state(&self) -> RngState;

    /// Restore state from saved state
    fn restore_state(&mut self, state: &RngState) -> crate::error::Result<()>;
}

// Implement StatefulRng for Xoshiro256Plus
#[cfg(feature = "fast_rng")]
impl crate::fast::xoshiro256::Xoshiro256Plus {
    /// Serialize the RNG state to JSON bytes
    #[cfg(feature = "serde")]
    pub fn serialize_state(&self) -> Result<Vec<u8>, serde_json::Error> {
        let state = self.save_state();
        serde_json::to_vec(&state)
    }

    /// Deserialize RNG state from JSON bytes
    #[cfg(feature = "serde")]
    pub fn deserialize_state(data: &[u8]) -> Result<Self, serde_json::Error> {
        let state: [u64; 4] = serde_json::from_slice(data)?;
        let mut rng = Self::new(0);
        rng.restore_state(state);
        Ok(rng)
    }
}

// Implement StatefulRng for PCG64
#[cfg(feature = "fast_rng")]
impl crate::fast::pcg64::Pcg64 {
    /// Serialize the RNG state to JSON bytes
    #[cfg(feature = "serde")]
    pub fn serialize_state(&self) -> Result<Vec<u8>, serde_json::Error> {
        let state = self.save_state();
        serde_json::to_vec(&state)
    }

    /// Deserialize RNG state from JSON bytes
    #[cfg(feature = "serde")]
    pub fn deserialize_state(data: &[u8]) -> Result<Self, serde_json::Error> {
        let state: [u64; 2] = serde_json::from_slice(data)?;
        let mut rng = Self::new(0);
        rng.restore_state(state);
        Ok(rng)
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use crate::fast::xoshiro256::Xoshiro256Plus;

    #[test]
    fn test_rng_state_serialization() {
        let rng = Xoshiro256Plus::new(42);
        let state = rng.save_state();

        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: [u64; 4] = serde_json::from_str(&serialized).unwrap();

        assert_eq!(state, deserialized);
    }
}
