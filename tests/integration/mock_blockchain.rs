//! Integration tests with mock blockchain state

#[cfg(test)]
#[cfg(feature = "custom_rng")]
mod tests {
    use clock_rand::*;

    struct MockBlock {
        hash: [u8; 32],
        timestamp: u64,
    }

    #[test]
    fn test_blockchain_seeding() {
        let block1 = MockBlock {
            hash: [0x01u8; 32],
            timestamp: 1000,
        };
        
        let block2 = MockBlock {
            hash: [0x02u8; 32],
            timestamp: 2000,
        };
        
        let mut rng = ChainSeedX::builder()
            .with_block_hash(block1.hash)
            .with_timestamp(block1.timestamp)
            .build()
            .unwrap();
        
        // Update with new block
        rng.update_block(&block2.hash, block2.timestamp).unwrap();
        
        // RNG should continue working
        let _ = <ChainSeedX as Rng>::next_u64(&mut rng);
    }

    #[test]
    fn test_vrf_integration() {
        let vrf_output = b"vrf_output_data";
        let block_hash = [0x42u8; 32];
        
        let rng = ChainSeedX::builder()
            .with_block_hash(block_hash)
            .with_vrf(vrf_output)
            .build()
            .unwrap();
        
        // Should be able to generate values
        let mut rng = rng;
        let _ = <ChainSeedX as Rng>::next_u64(&mut rng);
    }

    #[test]
    fn test_epoch_reseeding() {
        let mut rng = ChainSeedX::builder()
            .with_block_hash([0x01u8; 32])
            .build()
            .unwrap();
        
        // Simulate epoch change
        for i in 0..10 {
            let hash = [i as u8; 32];
            rng.update_block(&hash, 1000 + i as u64).unwrap();
        }
        
        // RNG should still work
        let _ = <ChainSeedX as Rng>::next_u64(&mut rng);
    }
}
