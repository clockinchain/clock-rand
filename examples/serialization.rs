//! Serialization example

#[cfg(all(feature = "serde", feature = "fast_rng"))]
use clock_rand::*;

#[cfg(all(feature = "serde", feature = "fast_rng"))]
fn main() {
    let mut rng = Xoshiro256Plus::new(42);
    let _ = rng.next_u64();

    // Save state
    let state = rng.save_state();
    let serialized = serde_json::to_string(&state).unwrap();
    println!("Serialized state: {}", serialized);

    // Restore state
    let deserialized: [u64; 4] = serde_json::from_str(&serialized).unwrap();
    let mut rng2 = Xoshiro256Plus::new(0);
    rng2.restore_state(deserialized);

    // Should continue from same point
    assert_eq!(rng.next_u64(), rng2.next_u64());
    println!("State restored successfully!");
}

#[cfg(not(all(feature = "serde", feature = "fast_rng")))]
fn main() {
    println!("This example requires the 'serde' and 'fast_rng' features");
}
