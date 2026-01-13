//! Fuzzing target for general RNG operations

#![no_main]

use libfuzzer_sys::fuzz_target;
use clock_rand::*;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    // Test with different RNG types based on input
    match data[0] % 4 {
        0 => {
            // Xoshiro256Plus
            let mut rng = Xoshiro256Plus::new(u64::from_le_bytes(data[1..9].try_into().unwrap_or([42; 8])));
            let _ = <Xoshiro256Plus as Rng>::next_u64(&mut rng);
            let _ = rng.next_u32();

            if data.len() > 16 {
                let mut buf = vec![0u8; data[16] as usize];
                rng.fill_bytes(&mut buf);
            }
        }
        1 => {
            // PCG64
            let mut rng = Pcg64::new(u64::from_le_bytes(data[1..9].try_into().unwrap_or([42; 8])));
            let _ = <Pcg64 as Rng>::next_u64(&mut rng);
            let _ = rng.next_u32();

            if data.len() > 16 {
                let mut buf = vec![0u8; data[16] as usize];
                rng.fill_bytes(&mut buf);
            }
        }
        2 => {
            // ChaCha20Rng (if crypto_rng feature enabled)
            #[cfg(feature = "crypto_rng")]
            {
                let seed_data = &data[1..];
                if let Ok(seed) = Seed::from_seed_slice(seed_data) {
                    let mut rng = ChaCha20Rng::from_seed(seed);
                    let _ = <ChaCha20Rng as Rng>::next_u64(&mut rng);
                    let _ = rng.next_u32();

                    if data.len() > 16 {
                        let mut buf = vec![0u8; data[16] as usize];
                        rng.fill_bytes(&mut buf);
                    }
                }
            }
        }
        3 => {
            // ChainSeedX (if custom_rng feature enabled)
            #[cfg(feature = "custom_rng")]
            {
                if data.len() >= 32 {
                    let hash: [u8; 32] = data[1..33].try_into().unwrap();
                    if let Ok(mut rng) = ChainSeedX::builder().with_block_hash(hash).build() {
                        let _ = <ChainSeedX as Rng>::next_u64(&mut rng);
                        let _ = rng.next_u32();

                        if data.len() > 64 {
                            let mut buf = vec![0u8; data[64] as usize];
                            rng.fill_bytes(&mut buf);
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
});