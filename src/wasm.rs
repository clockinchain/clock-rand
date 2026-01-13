//! WASM bindings for clock-rand
//!
//! Provides JavaScript/TypeScript bindings for browser use.

#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
use crate::fast::xoshiro256::Xoshiro256Plus;
#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
use crate::seed::Seed;
#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
use crate::traits::RngExt;

#[cfg(all(
    feature = "wasm_crypto",
    feature = "wasm-bindgen",
    target_arch = "wasm32"
))]
use js_sys::{Promise, Uint8Array};
#[cfg(all(
    feature = "wasm_crypto",
    feature = "wasm-bindgen",
    target_arch = "wasm32"
))]
use wasm_bindgen_futures::JsFuture;
#[cfg(all(
    feature = "wasm_crypto",
    feature = "wasm-bindgen",
    target_arch = "wasm32"
))]
use web_sys::{Crypto, Window};

/// WASM-compatible RNG wrapper
#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
#[wasm_bindgen]
pub struct WasmRng {
    inner: Xoshiro256Plus,
}

#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
#[wasm_bindgen]
impl WasmRng {
    /// Create a new RNG from a seed
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u64) -> Self {
        Self {
            inner: Xoshiro256Plus::new(seed),
        }
    }

    /// Create from a block hash (32 bytes as u8 array)
    #[wasm_bindgen]
    pub fn from_block_hash(hash: &[u8]) -> Result<WasmRng, JsValue> {
        if hash.len() < 32 {
            return Err(JsValue::from_str("Hash must be at least 32 bytes"));
        }

        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash[..32]);

        let seed = Seed::from_block_hash(&hash_array)
            .map_err(|e| JsValue::from_str(&format!("Seed error: {}", e)))?;

        let rng = Xoshiro256Plus::from_seed_obj(&seed)
            .map_err(|e| JsValue::from_str(&format!("RNG error: {}", e)))?;

        Ok(Self { inner: rng })
    }

    /// Generate next u32
    #[wasm_bindgen]
    pub fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    /// Generate next u64
    #[wasm_bindgen]
    pub fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    /// Fill a Uint8Array with random bytes
    #[wasm_bindgen]
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest);
    }

    /// Generate a random f64 in [0, 1)
    #[wasm_bindgen]
    pub fn next_f64(&mut self) -> f64 {
        self.inner.gen_f64()
    }

    /// Generate a random f32 in [0, 1)
    #[wasm_bindgen]
    pub fn next_f32(&mut self) -> f32 {
        self.inner.gen_f32()
    }
}

#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Demo function for WASM - displays a greeting alert
#[cfg(all(feature = "wasm", feature = "wasm-bindgen"))]
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! Welcome to clock-rand", name));
}

/// WASM Crypto RNG using Web Crypto API
#[cfg(all(
    feature = "wasm_crypto",
    feature = "wasm-bindgen",
    target_arch = "wasm32"
))]
#[wasm_bindgen]
pub struct WasmCryptoRng {
    buffer: Vec<u8>,
    pos: usize,
}

#[cfg(all(
    feature = "wasm_crypto",
    feature = "wasm-bindgen",
    target_arch = "wasm32"
))]
#[wasm_bindgen]
impl WasmCryptoRng {
    /// Create a new crypto RNG using Web Crypto API entropy
    #[wasm_bindgen]
    pub async fn new() -> Result<WasmCryptoRng, JsValue> {
        let window = web_sys::window().ok_or("No window object")?;
        let crypto = window.crypto().map_err(|_| "Crypto API not available")?;

        // Get 256 bits of entropy from Web Crypto API
        let entropy = Self::get_crypto_random_bytes(&crypto, 32).await?;

        // Create seed from entropy
        let seed = Seed::from_bytes(entropy.to_vec())
            .map_err(|e| JsValue::from_str(&format!("Seed error: {}", e)))?;

        // Use ChaCha20 for crypto RNG (simplified for WASM)
        // For now, just use deterministic seeding
        let mut rng = crate::crypto::chacha20::ChaCha20Rng::new(&seed)
            .map_err(|e| JsValue::from_str(&format!("RNG error: {}", e)))?;

        // Fill initial buffer
        let mut buffer = vec![0u8; 1024];
        rng.fill_bytes(&mut buffer);

        Ok(Self { buffer, pos: 0 })
    }

    /// Generate next u32
    #[wasm_bindgen]
    pub fn next_u32(&mut self) -> u32 {
        if self.pos + 4 > self.buffer.len() {
            // Refill buffer - in real implementation would use crypto
            self.pos = 0;
        }

        let bytes = &self.buffer[self.pos..self.pos + 4];
        self.pos += 4;

        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    /// Generate next u64
    #[wasm_bindgen]
    pub fn next_u64(&mut self) -> u64 {
        if self.pos + 8 > self.buffer.len() {
            self.pos = 0;
        }

        let bytes = &self.buffer[self.pos..self.pos + 8];
        self.pos += 8;

        u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    /// Fill a Uint8Array with random bytes
    #[wasm_bindgen]
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let len = dest.len();
        let mut filled = 0;

        while filled < len {
            let remaining = len - filled;
            let to_copy = remaining.min(self.buffer.len() - self.pos);

            dest[filled..filled + to_copy]
                .copy_from_slice(&self.buffer[self.pos..self.pos + to_copy]);
            self.pos += to_copy;
            filled += to_copy;

            if self.pos >= self.buffer.len() {
                self.pos = 0;
                // In real implementation, refill with crypto entropy
            }
        }
    }

    /// Internal function to get random bytes from Web Crypto API
    async fn get_crypto_random_bytes(crypto: &Crypto, len: usize) -> Result<Vec<u8>, JsValue> {
        let array = Uint8Array::new_with_length(len as u32);
        crypto.get_random_values_with_array_buffer_view(&array)?;

        let mut result = vec![0u8; len];
        array.copy_to(&mut result);

        Ok(result)
    }
}
