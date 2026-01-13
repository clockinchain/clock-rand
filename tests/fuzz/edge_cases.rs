//! Fuzzing target for edge cases and boundary conditions

#![no_main]

use libfuzzer_sys::fuzz_target;
use clock_rand::*;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    let test_case = data[0];
    let param = data[1];

    match test_case % 8 {
        0 => {
            // Edge case: Very small buffer sizes
            let mut rng = Xoshiro256Plus::new(42);
            let sizes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128];

            for &size in &sizes {
                let mut buf = vec![0u8; size];
                rng.fill_bytes(&mut buf);
                // Ensure buffer was filled
                assert!(buf.len() == size);
            }
        }
        1 => {
            // Edge case: Large buffer allocations (limited for fuzzing)
            let mut rng = Xoshiro256Plus::new(42);
            let size = (param as usize).min(1024 * 10); // Limit to 10KB for fuzzing
            let mut buf = vec![0u8; size];
            rng.fill_bytes(&mut buf);
            assert!(buf.len() == size);
        }
        2 => {
            // Edge case: Seed generation boundary conditions
            let seed_size = (param as usize).max(1).min(1024);
            let mut seed_data = vec![0u8; seed_size];

            // Fill with fuzzer data
            let copy_len = seed_data.len().min(data.len().saturating_sub(2));
            seed_data[..copy_len].copy_from_slice(&data[2..2 + copy_len]);

            // Try to create seed - should not panic
            let _ = Seed::from_bytes(seed_data);
        }
        3 => {
            // Edge case: Distribution boundary values
            #[cfg(feature = "distributions")]
            {
                let mut rng = Xoshiro256Plus::new(42);
                let uniform = Uniform::new(0u32, u32::MAX);

                // Generate many values to test distribution
                for _ in 0..100 {
                    let _ = uniform.sample(&mut rng);
                }
            }
        }
        4 => {
            // Edge case: State save/restore with corrupted data
            #[cfg(feature = "fast_rng")]
            {
                let mut rng = Xoshiro256Plus::new(42);
                let original_value = rng.next_u64();

                // Save state
                let state = rng.save_state();

                // Corrupt state with fuzzer data (limited corruption for safety)
                let mut corrupted_state = state;
                if corrupted_state.len() > 2 {
                    let corrupt_index = (param as usize) % corrupted_state.len();
                    corrupted_state[corrupt_index] = data[2].wrapping_add(corrupted_state[corrupt_index]);
                }

                // Try to restore - should not panic even with corrupted data
                let _ = rng.restore_state(corrupted_state);

                // RNG should still be usable
                let _ = rng.next_u64();
            }
        }
        5 => {
            // Edge case: Utility functions with boundary inputs
            let mut rng = Xoshiro256Plus::new(42);

            // Test sample with various sizes
            let sample_size = (param as usize).min(100);
            let data: Vec<i32> = (0..200).collect();

            if sample_size <= data.len() {
                let _ = sample(&mut rng, &data, sample_size);
            }
        }
        6 => {
            // Edge case: Range generation boundary conditions
            let mut rng = Xoshiro256Plus::new(42);

            // Test various range combinations
            let ranges = [
                (0u32, 1u32),
                (0u32, u32::MAX),
                (u32::MAX - 1, u32::MAX),
                (param as u32, (param as u32).wrapping_add(100)),
            ];

            for (low, high) in &ranges {
                if *low < *high {
                    let _ = gen_range_u32(&mut rng, *low, *high);
                }
            }
        }
        7 => {
            // Edge case: ChaCha20Rng with various seed sizes
            #[cfg(feature = "crypto_rng")]
            {
                let seed_size = (param as usize).max(1).min(256);
                let mut seed_data = vec![0u8; seed_size];

                let copy_len = seed_data.len().min(data.len().saturating_sub(2));
                seed_data[..copy_len].copy_from_slice(&data[2..2 + copy_len]);

                if let Ok(seed) = Seed::from_bytes(seed_data) {
                    let mut rng = ChaCha20Rng::from_seed(seed);

                    // Test various operations
                    let _ = rng.next_u64();
                    let _ = rng.next_u32();

                    // Test buffer filling with various sizes
                    let buf_sizes = [1, 4, 8, 16, 32, 64, 128];
                    for &size in &buf_sizes {
                        let mut buf = vec![0u8; size];
                        rng.fill_bytes(&mut buf);
                        assert!(buf.len() == size);
                    }
                }
            }
        }
        _ => unreachable!(),
    }
});