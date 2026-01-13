//! WASM example for clock-rand
//!
//! This example demonstrates how to use clock-rand in WebAssembly.

use clock_rand::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Generate random numbers using fast RNG
#[wasm_bindgen]
pub fn generate_fast_random(count: usize) -> Vec<u32> {
    let mut rng = Xoshiro256Plus::new(12345);
    let mut result = Vec::with_capacity(count);

    for _ in 0..count {
        result.push(rng.next_u32());
    }

    result
}

/// Generate random numbers using crypto RNG
#[cfg(feature = "crypto_rng")]
#[wasm_bindgen]
pub fn generate_crypto_random(count: usize) -> Vec<u32> {
    let seed = Seed::from_seed_slice(b"wasm_crypto_seed_1234567890").unwrap();
    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut result = Vec::with_capacity(count);

    for _ in 0..count {
        result.push(rng.next_u32());
    }

    result
}

/// Generate random numbers using ChainSeed-X
#[cfg(feature = "custom_rng")]
#[wasm_bindgen]
pub fn generate_blockchain_random(block_hash: &[u8], count: usize) -> Result<Vec<u32>, JsValue> {
    if block_hash.len() != 32 {
        return Err(JsValue::from_str("Block hash must be 32 bytes"));
    }

    let hash: [u8; 32] = block_hash.try_into().unwrap();
    let mut rng = ChainSeedX::builder()
        .with_block_hash(hash)
        .with_timestamp(js_sys::Date::now() as u64)
        .build()
        .map_err(|e| JsValue::from_str(&format!("Failed to create RNG: {:?}", e)))?;

    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(rng.next_u32());
    }

    Ok(result)
}

/// Benchmark fast RNG performance
#[wasm_bindgen]
pub fn benchmark_fast_rng(iterations: usize) -> f64 {
    let start = js_sys::Date::now();
    let mut rng = Xoshiro256Plus::new(42);

    for _ in 0..iterations {
        let _ = <Xoshiro256Plus as Rng>::next_u64(&mut rng);
    }

    let end = js_sys::Date::now();
    end - start
}

/// Get RNG information
#[wasm_bindgen]
pub fn get_rng_info() -> String {
    format!(
        "Clock-Rand WASM Example\\n\
         Fast RNG: Xoshiro256+\\n\
         Crypto RNG: ChaCha20\\n\
         Custom RNG: ChainSeed-X\\n\
         Features: fast_rng={}, crypto_rng={}, custom_rng={}",
        cfg!(feature = "fast_rng"),
        cfg!(feature = "crypto_rng"),
        cfg!(feature = "custom_rng")
    )
}