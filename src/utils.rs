//! Utility functions for RNG operations

#[cfg(feature = "distributions")]
use crate::distributions::{Distribution, Uniform};
use crate::traits::{Rng, RngExt};

/// Generate a random u32 value in the given range [low, high)
#[cfg(feature = "distributions")]
pub fn gen_range_u32<R: Rng>(rng: &mut R, low: u32, high: u32) -> u32 {
    let dist = Uniform::new_u32(low, high);
    dist.sample(rng)
}

/// Generate a random u32 value in the given range [low, high) - fallback without distributions
#[cfg(not(feature = "distributions"))]
pub fn gen_range_u32<R: Rng>(rng: &mut R, low: u32, high: u32) -> u32 {
    if low >= high {
        panic!("gen_range_u32: low must be less than high");
    }
    let range = high - low;
    let random_value = rng.next_u32() % range;
    low + random_value
}

/// Generate a random u64 value in the given range [low, high)
#[cfg(feature = "distributions")]
pub fn gen_range_u64<R: Rng>(rng: &mut R, low: u64, high: u64) -> u64 {
    let dist = Uniform::new_u64(low, high);
    dist.sample(rng)
}

/// Generate a random u64 value in the given range [low, high) - fallback without distributions
#[cfg(not(feature = "distributions"))]
pub fn gen_range_u64<R: Rng>(rng: &mut R, low: u64, high: u64) -> u64 {
    if low >= high {
        panic!("gen_range_u64: low must be less than high");
    }
    let range = high - low;
    let random_value = rng.next_u64() % range;
    low + random_value
}

/// Generate a random f32 value in the given range [low, high)
#[cfg(feature = "distributions")]
pub fn gen_range_f32<R: Rng>(rng: &mut R, low: f32, high: f32) -> f32 {
    let dist = Uniform::new_f32(low, high);
    dist.sample(rng)
}

/// Generate a random f32 value in the given range [low, high) - fallback without distributions
#[cfg(not(feature = "distributions"))]
pub fn gen_range_f32<R: Rng>(rng: &mut R, low: f32, high: f32) -> f32 {
    if low >= high {
        panic!("gen_range_f32: low must be less than high");
    }
    let range = high - low;
    let random_value = rng.gen_f32() * range;
    low + random_value
}

/// Generate a random f64 value in the given range [low, high)
#[cfg(feature = "distributions")]
pub fn gen_range_f64<R: Rng>(rng: &mut R, low: f64, high: f64) -> f64 {
    let dist = Uniform::new_f64(low, high);
    dist.sample(rng)
}

/// Generate a random f64 value in the given range [low, high) - fallback without distributions
#[cfg(not(feature = "distributions"))]
pub fn gen_range_f64<R: Rng>(rng: &mut R, low: f64, high: f64) -> f64 {
    if low >= high {
        panic!("gen_range_f64: low must be less than high");
    }
    let range = high - low;
    let random_value = rng.gen_f64() * range;
    low + random_value
}

/// Generate a random value in the given range [low, high) - fallback without distributions
#[cfg(not(feature = "distributions"))]
pub fn gen_range<R: Rng>(rng: &mut R, low: u64, high: u64) -> u64 {
    if low >= high {
        panic!("gen_range: low must be less than high");
    }
    let range = high - low;
    let random_value = rng.next_u64() % range;
    low + random_value
}

/// Fill a byte buffer with random data
pub fn fill_bytes<R: Rng>(rng: &mut R, buf: &mut [u8]) {
    rng.fill_bytes(buf);
}

/// Generate a random f32 in [0.0, 1.0)
pub fn gen_f32<R: Rng>(rng: &mut R) -> f32 {
    rng.gen_f32()
}

/// Generate a random f64 in [0.0, 1.0)
pub fn gen_f64<R: Rng>(rng: &mut R) -> f64 {
    rng.gen_f64()
}

/// Shuffle a slice in place using Fisher-Yates algorithm
pub fn shuffle<R: Rng, T>(rng: &mut R, slice: &mut [T]) {
    for i in (1..slice.len()).rev() {
        let j = gen_range_u32(rng, 0, (i + 1) as u32) as usize;
        slice.swap(i, j);
    }
}

/// Sample n elements from a slice (without replacement)
#[cfg(feature = "std")]
pub fn sample<R: Rng, T: Clone>(rng: &mut R, slice: &[T], n: usize) -> Vec<T> {
    if n >= slice.len() {
        return slice.to_vec();
    }

    let mut indices: Vec<usize> = (0..slice.len()).collect();
    shuffle(rng, &mut indices);
    indices[..n].iter().map(|&i| slice[i].clone()).collect()
}

/// Choose a random element from a slice
pub fn choose<'a, R: Rng, T>(rng: &mut R, slice: &'a [T]) -> Option<&'a T> {
    if slice.is_empty() {
        return None;
    }
    #[cfg(feature = "distributions")]
    {
        let index: usize = gen_range_u64(rng, 0, slice.len() as u64) as usize;
        Some(&slice[index])
    }
    #[cfg(not(feature = "distributions"))]
    {
        let len = slice.len() as u64;
        let random_value = rng.next_u64() % len;
        let index = random_value as usize;
        Some(&slice[index])
    }
}

/// Weighted random selection
pub fn weighted_choose<'a, R: Rng, T>(
    rng: &mut R,
    items: &'a [T],
    weights: &[f64],
) -> Option<&'a T> {
    if items.is_empty() || items.len() != weights.len() {
        return None;
    }

    let total_weight: f64 = weights.iter().sum();
    if total_weight <= 0.0 {
        return None;
    }

    let mut r = gen_f64(rng) * total_weight;
    for (item, &weight) in items.iter().zip(weights.iter()) {
        r -= weight;
        if r <= 0.0 {
            return Some(item);
        }
    }

    // Fallback to last item (shouldn't happen with proper weights)
    items.last()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fast::xoshiro256::Xoshiro256Plus;

    #[test]
    fn test_gen_range_u64() {
        let mut rng = Xoshiro256Plus::new(42);
        for _ in 0..100 {
            let x = gen_range_u64(&mut rng, 10, 20);
            assert!(x >= 10 && x < 20);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_shuffle() {
        let mut rng = Xoshiro256Plus::new(42);
        let mut vec = vec![1, 2, 3, 4, 5];
        let original = vec.clone();
        shuffle(&mut rng, &mut vec);

        // Should be a permutation
        assert_eq!(vec.len(), original.len());
        assert!(vec.iter().all(|&x| original.contains(&x)));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_sample() {
        let mut rng = Xoshiro256Plus::new(42);
        let vec = vec![1, 2, 3, 4, 5];
        let sampled = sample(&mut rng, &vec, 3);

        assert_eq!(sampled.len(), 3);
        assert!(sampled.iter().all(|&x| vec.contains(&x)));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_choose() {
        let mut rng = Xoshiro256Plus::new(42);
        let vec = vec![1, 2, 3, 4, 5];
        let chosen = choose(&mut rng, &vec);

        assert!(chosen.is_some());
        assert!(vec.contains(chosen.unwrap()));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_weighted_choose() {
        let mut rng = Xoshiro256Plus::new(42);
        let items = vec!["a", "b", "c"];
        let weights = vec![0.5, 0.3, 0.2];
        let chosen = weighted_choose(&mut rng, &items, &weights);

        assert!(chosen.is_some());
        assert!(items.contains(chosen.unwrap()));
    }
}
