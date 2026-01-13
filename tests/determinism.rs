//! Deterministic seeding and replay tests

#[cfg(test)]
mod tests {
    use clock_rand::*;

    #[test]
    #[cfg(feature = "fast_rng")]
    fn test_deterministic_seeding() {
        // Same seed should produce same sequence
        let mut rng1 = Xoshiro256Plus::new(12345);
        let mut rng2 = Xoshiro256Plus::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_seed_from_block_hash() {
        let hash = [0x42u8; 32];
        let seed1 = Seed::from_block_hash(&hash).unwrap();
        let seed2 = Seed::from_block_hash(&hash).unwrap();

        assert_eq!(seed1.as_bytes(), seed2.as_bytes());
    }

    #[test]
    fn test_seed_from_timestamp() {
        let timestamp = 1234567890u64;
        let seed1 = Seed::from_timestamp(timestamp).unwrap();
        let seed2 = Seed::from_timestamp(timestamp).unwrap();

        assert_eq!(seed1.as_bytes(), seed2.as_bytes());
    }

    #[test]
    #[cfg(feature = "fast_rng")]
    fn test_cross_platform_determinism() {
        // Test that same seed produces same output across different operations
        let seed = Seed::from_timestamp(42).unwrap();
        let mut rng1 = Xoshiro256Plus::from_seed_obj(&seed).unwrap();
        let mut rng2 = Xoshiro256Plus::from_seed_obj(&seed).unwrap();

        // Generate some values
        let v1 = rng1.next_u64();
        let v2 = rng2.next_u64();
        assert_eq!(v1, v2);

        // Jump and verify
        rng1.jump();
        rng2.jump();
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }

    #[test]
    #[cfg(feature = "fast_rng")]
    fn test_save_restore_state() {
        let mut rng1 = Xoshiro256Plus::new(42);
        let _ = rng1.next_u64();
        let state = rng1.save_state();

        let mut rng2 = Xoshiro256Plus::new(0);
        rng2.restore_state(state);

        // Should continue from same point
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }

    #[test]
    #[cfg(feature = "crypto_rng")]
    fn test_crypto_rng_deterministic() {
        let seed =
            Seed::from_bytes(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
        let mut rng1 = Blake3Drbg::new(&seed).unwrap();
        let mut rng2 = Blake3Drbg::new(&seed).unwrap();

        for _ in 0..10 {
            assert_eq!(
                clock_rand::Rng::next_u64(&mut rng1),
                clock_rand::Rng::next_u64(&mut rng2)
            );
        }
    }
}
