//! Thread-safe usage example

#[cfg(feature = "thread_safe")]
use clock_rand::thread_safe::ThreadSafeRng;
#[cfg(feature = "thread_safe")]
use clock_rand::*;
#[cfg(feature = "thread_safe")]
use std::thread;

#[cfg(feature = "thread_safe")]
fn main() {
    let rng = ThreadSafeRng::new(Xoshiro256Plus::new(42));
    let rng_clone = rng.clone();

    // Use RNG from multiple threads
    let handle = thread::spawn(move || {
        for _ in 0..10 {
            println!("Thread 1: {}", rng_clone.next_u32());
        }
    });

    for _ in 0..10 {
        println!("Main thread: {}", rng.next_u32());
    }

    handle.join().unwrap();
}

#[cfg(not(feature = "thread_safe"))]
fn main() {
    println!("This example requires the 'thread_safe' feature");
}
