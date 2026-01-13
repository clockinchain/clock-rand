//! Fuzzing target for RNG state corruption and restoration

#![no_main]

use libfuzzer_sys::fuzz_target;
use clock_rand::*;

fuzz_target!(|data: &[u8]| {
    // Test Xoshiro256Plus state corruption
    if data.len() >= 32 {
        let mut rng = Xoshiro256Plus::new(12345);

        // Corrupt state with fuzzer input
        let state: [u64; 4] = [
            u64::from_le_bytes(data[0..8].try_into().unwrap_or([0; 8])),
            u64::from_le_bytes(data[8..16].try_into().unwrap_or([0; 8])),
            u64::from_le_bytes(data[16..24].try_into().unwrap_or([0; 8])),
            u64::from_le_bytes(data[24..32].try_into().unwrap_or([0; 8])),
        ];

        // Test restoration (should not panic)
        let _ = rng.restore_state(state);

        // Generate some values to test state validity
        let _ = <Xoshiro256Plus as Rng>::next_u64(&mut rng);
        let _ = rng.next_u32();
    }

    // Test PCG64 state corruption
    if data.len() >= 16 {
        let mut rng = Pcg64::new(12345);

        let state: [u64; 2] = [
            u64::from_le_bytes(data[0..8].try_into().unwrap_or([0; 8])),
            u64::from_le_bytes(data[8..16].try_into().unwrap_or([0; 8])),
        ];

        let _ = rng.restore_state(state);
        let _ = <Pcg64 as Rng>::next_u64(&mut rng);
    }
});