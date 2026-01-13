//! Blockchain seeding example

use clock_rand::*;

fn main() {
    // Seed from block hash
    let block_hash = [0x42u8; 32];
    let seed = Seed::from_block_hash(&block_hash).unwrap();
    let mut rng = Xoshiro256Plus::from_seed_obj(&seed).unwrap();

    println!("RNG seeded from block hash");
    println!("Random value: {}", rng.next_u64());

    // Seed from blockchain state
    let timestamp = 1234567890u64;
    let seed2 = Seed::from_blockchain_state(&block_hash, timestamp, None).unwrap();
    let mut rng2 = Xoshiro256Plus::from_seed_obj(&seed2).unwrap();

    println!("RNG seeded from blockchain state");
    println!("Random value: {}", rng2.next_u64());
}
