//! Fuzzing target for fork detection and reseeding

#![no_main]

use libfuzzer_sys::fuzz_target;
use clock_rand::*;

fuzz_target!(|data: &[u8]| {
    // Only test with custom_rng feature
    #[cfg(feature = "custom_rng")]
    {
        if data.len() >= 32 {
            let hash1: [u8; 32] = data[..32].try_into().unwrap();

            let mut rng = ChainSeedX::builder()
                .with_block_hash(hash1)
                .with_fork_detection(true)
                .build()
                .unwrap();

            // Generate some values first
            let _ = <ChainSeedX as Rng>::next_u64(&mut rng);
            let _ = rng.next_u32();

            // Test fork detection with fuzzer input as new hash
            let _ = rng.check_fork(&hash1); // Same hash (no fork)
            let _ = rng.check_fork(data); // Different hash (potential fork)

            // Continue generating values
            let _ = <ChainSeedX as Rng>::next_u64(&mut rng);
        }
    }
});