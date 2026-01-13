//! Fuzzing target for seed parsing and validation

#![no_main]

use libfuzzer_sys::fuzz_target;
use clock_rand::*;

fuzz_target!(|data: &[u8]| {
    // Test seed parsing with arbitrary input
    let _ = Seed::from_seed_slice(data);

    // Test block hash parsing
    if data.len() >= 32 {
        let hash: [u8; 32] = data[..32].try_into().unwrap();
        let _ = Seed::from_block_hash(&hash);
    }

    // Test combined seed parsing
    let sources = vec![data];
    let _ = Seed::from_combined(&sources);
});