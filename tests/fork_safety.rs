//! Fork detection and reseeding tests

#[cfg(test)]
#[cfg(feature = "custom_rng")]
mod tests {
    use clock_rand::*;

    #[test]
    fn test_fork_detection() {
        let hash1 = [0x01u8; 32];
        let hash2 = [0x02u8; 32];

        let mut rng = ChainSeedX::builder()
            .with_block_hash(hash1)
            .with_fork_detection(true)
            .build()
            .unwrap();

        // Different hash should trigger fork
        let fork_detected = rng.check_fork(&hash2).unwrap();
        assert!(fork_detected);
    }

    #[test]
    fn test_fork_reseeding() {
        let hash1 = [0x01u8; 32];
        let hash2 = [0x02u8; 32];

        let mut rng1 = ChainSeedX::builder()
            .with_block_hash(hash1)
            .build()
            .unwrap();

        let mut rng2 = ChainSeedX::builder()
            .with_block_hash(hash1)
            .build()
            .unwrap();

        // Generate some values
        let v1 = <ChainSeedX as Rng>::next_u64(&mut rng1);
        let v2 = <ChainSeedX as Rng>::next_u64(&mut rng2);
        assert_eq!(v1, v2);

        // Fork detected on rng1
        let fork_detected = rng1.check_fork(&hash2).unwrap();
        assert!(fork_detected);

        // Now they should diverge
        assert_ne!(
            <ChainSeedX as Rng>::next_u64(&mut rng1),
            <ChainSeedX as Rng>::next_u64(&mut rng2)
        );
    }

    #[test]
    fn test_update_block() {
        let hash1 = [0x01u8; 32];
        let hash2 = [0x02u8; 32];

        let mut rng = ChainSeedX::builder()
            .with_block_hash(hash1)
            .build()
            .unwrap();

        // Generate a value before update
        let value_before = <ChainSeedX as Rng>::next_u64(&mut rng);

        // Update with new block (no fork since it's the first block)
        rng.update_block(&hash2, 12345).unwrap();

        // RNG should still work and produce different values due to reseeding
        let value_after = <ChainSeedX as Rng>::next_u64(&mut rng);
        assert_ne!(value_before, value_after);
    }
}
