//! Basic usage example

use clock_rand::*;

fn main() {
    // Fast RNG
    let mut rng = Xoshiro256Plus::new(42);
    println!("Random u64: {}", rng.next_u64());
    println!("Random u32: {}", rng.next_u32());
    println!("Random f64: {}", rng.gen_f64());

    // Generate random bytes
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    println!("Random bytes: {:?}", &bytes[..8]);

    // Use utility functions
    let value = gen_range_u64(&mut rng, 10, 20);
    println!("Random in range [10, 20): {}", value);
}
