//! Fork detection example

#[cfg(feature = "custom_rng")]
use clock_rand::*;

#[cfg(feature = "custom_rng")]
fn main() {
    // Create RNG with fork detection
    let mut rng = ChainSeedX::builder()
        .with_block_hash([0x01u8; 32])
        .with_timestamp(1000)
        .with_fork_detection(true)
        .build()
        .unwrap();

    println!("Initial RNG state");
    let v1 = <ChainSeedX as Rng>::next_u64(&mut rng);
    println!("Random value: {}", v1);

    // Simulate fork
    let fork_hash = [0x02u8; 32];
    let fork_detected = rng.check_fork(&fork_hash).unwrap();

    if fork_detected {
        println!("Fork detected! RNG reseeded.");
    }

    // Continue generating
    let v2 = <ChainSeedX as Rng>::next_u64(&mut rng);
    println!("Random value after fork: {}", v2);
}

#[cfg(not(feature = "custom_rng"))]
fn main() {
    println!("This example requires the 'custom_rng' feature");
}
