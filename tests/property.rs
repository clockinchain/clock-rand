//! Property-based tests

#[cfg(test)]
#[cfg(feature = "fast_rng")]
mod tests {
    use clock_rand::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_seed_generation(seed_val in 0u64..u64::MAX) {
            let mut rng1 = Xoshiro256Plus::new(seed_val);
            let mut rng2 = Xoshiro256Plus::new(seed_val);

            // Same seed should produce same sequence
            for _ in 0..10 {
                prop_assert_eq!(rng1.next_u64(), rng2.next_u64());
            }
        }

        #[test]
        fn test_range_generation(
            low in 0u64..1000u64,
            high in 1000u64..2000u64
        ) {
            prop_assume!(low < high);
            let mut rng = Xoshiro256Plus::new(42);

            for _ in 0..100 {
                let value = clock_rand::gen_range_u64(&mut rng, low, high);
                prop_assert!(value >= low && value < high);
            }
        }

        #[test]
        fn test_state_transitions(seed_val in 0u64..u64::MAX) {
            let mut rng = Xoshiro256Plus::new(seed_val);
            let state1 = rng.save_state();

            let _ = rng.next_u64();
            let state2 = rng.save_state();

            // States should be different after generation
            prop_assert_ne!(state1, state2);

            // Restoring state1 should allow replay
            rng.restore_state(state1);
            let v1 = rng.next_u64();
            rng.restore_state(state1);
            let v2 = rng.next_u64();
            prop_assert_eq!(v1, v2);
        }
    }
}
