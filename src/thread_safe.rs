//! Thread-safe wrappers for RNGs
//!
//! Provides synchronized access to RNGs for multi-threaded applications.

#[cfg(feature = "thread_safe")]
use crate::error::Result;
#[cfg(feature = "thread_safe")]
use crate::traits::Rng;

#[cfg(feature = "thread_safe")]
use std::sync::{Arc, Mutex, RwLock};

/// Thread-safe wrapper for any RNG
#[cfg(feature = "thread_safe")]
pub struct ThreadSafeRng<T> {
    inner: Arc<Mutex<T>>,
}

#[cfg(feature = "thread_safe")]
impl<T: Rng> ThreadSafeRng<T> {
    /// Create a new thread-safe wrapper
    pub fn new(rng: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(rng)),
        }
    }

    /// Generate the next u32 (thread-safe)
    pub fn next_u32(&self) -> u32 {
        let mut rng = self.inner.lock().unwrap();
        rng.next_u32()
    }

    /// Generate the next u64 (thread-safe)
    pub fn next_u64(&self) -> u64 {
        let mut rng = self.inner.lock().unwrap();
        rng.next_u64()
    }

    /// Fill bytes (thread-safe)
    pub fn fill_bytes(&self, dest: &mut [u8]) {
        let mut rng = self.inner.lock().unwrap();
        rng.fill_bytes(dest);
    }
}

#[cfg(feature = "thread_safe")]
impl<T: Rng> Rng for ThreadSafeRng<T> {
    fn next_u32(&mut self) -> u32 {
        ThreadSafeRng::next_u32(self)
    }

    fn next_u64(&mut self) -> u64 {
        ThreadSafeRng::next_u64(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        ThreadSafeRng::fill_bytes(self, dest);
    }
}

#[cfg(feature = "thread_safe")]
impl<T> Clone for ThreadSafeRng<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Fork-safe wrapper that detects chain forks and reseeds
#[cfg(all(feature = "thread_safe", feature = "fork_safe"))]
pub struct ForkSafeRng<T> {
    inner: Arc<RwLock<T>>,
    block_hashes: Arc<RwLock<Vec<[u8; 32]>>>,
    fork_history_size: usize,
}

#[cfg(all(feature = "thread_safe", feature = "fork_safe"))]
impl<T: crate::traits::SeedableRng<Seed = crate::seed::Seed>> ForkSafeRng<T> {
    /// Create a new fork-safe wrapper
    pub fn new(rng: T, fork_history_size: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(rng)),
            block_hashes: Arc::new(RwLock::new(Vec::new())),
            fork_history_size,
        }
    }

    /// Check for fork and reseed if necessary
    pub fn check_fork(&self, current_hash: &[u8; 32]) -> Result<bool> {
        let mut hashes = self.block_hashes.write().unwrap();

        // Check if current hash matches expected
        if let Some(last_hash) = hashes.last()
            && last_hash == current_hash
        {
            return Ok(false); // No fork
        }

        // Fork detected - reseed
        hashes.push(*current_hash);

        // Keep only recent history
        if hashes.len() > self.fork_history_size {
            hashes.remove(0);
        }

        // Reseed RNG
        let seed = crate::seed::Seed::from_block_hash(current_hash)?;
        let mut rng = self.inner.write().unwrap();
        rng.reseed(seed)?;

        Ok(true) // Fork detected and handled
    }
}

#[cfg(all(feature = "thread_safe", feature = "fork_safe"))]
impl<T: Rng> Rng for ForkSafeRng<T> {
    fn next_u32(&mut self) -> u32 {
        let mut rng = self.inner.write().unwrap();
        rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        let mut rng = self.inner.write().unwrap();
        rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut rng = self.inner.write().unwrap();
        rng.fill_bytes(dest);
    }
}

/// Global RNG instance (like rand::thread_rng)
#[cfg(feature = "std")]
pub struct GlobalRng {
    // Implementation would use thread-local storage
    // For now, this is a placeholder
}

#[cfg(feature = "std")]
impl GlobalRng {
    /// Get the global RNG instance
    pub fn thread_rng() -> Self {
        Self {}
    }
}

#[cfg(feature = "std")]
impl Rng for GlobalRng {
    fn next_u32(&mut self) -> u32 {
        // Would use thread-local RNG
        0 // Placeholder
    }

    fn next_u64(&mut self) -> u64 {
        // Would use thread-local RNG
        0 // Placeholder
    }

    fn fill_bytes(&mut self, _dest: &mut [u8]) {
        // Would use thread-local RNG
    }
}
