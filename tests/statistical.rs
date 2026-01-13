//! Statistical quality tests for RNGs

#[cfg(test)]
mod tests {
    use clock_rand::*;

    #[test]
    #[cfg(feature = "fast_rng")]
    fn test_chi_square() {
        // Simple chi-square test for uniformity
        let mut rng = Xoshiro256Plus::new(42);
        let mut buckets = [0u64; 10];

        // Generate 10000 values and bucket them
        for _ in 0..10000 {
            let value = rng.next_u64();
            let bucket = (value % 10) as usize;
            buckets[bucket] += 1;
        }

        // Expected count per bucket
        let expected = 10000 / 10;
        let mut chi_square = 0.0;

        for &count in &buckets {
            let diff = count as f64 - expected as f64;
            chi_square += (diff * diff) / expected as f64;
        }

        // Chi-square critical value for 9 degrees of freedom at 0.05 level is ~16.92
        assert!(chi_square < 20.0, "Chi-square test failed: {}", chi_square);
    }

    #[test]
    #[cfg(feature = "fast_rng")]
    fn test_runs() {
        // Runs test: count sequences of consecutive same bits
        let mut rng = Xoshiro256Plus::new(42);
        let mut bits = Vec::new();

        // Generate 1000 bits
        for _ in 0..1000 {
            bits.push((rng.next_u64() & 1) == 1);
        }

        // Count runs
        let mut runs = 1;
        for i in 1..bits.len() {
            if bits[i] != bits[i - 1] {
                runs += 1;
            }
        }

        // Expected runs for random sequence: ~500
        // Allow some variance
        assert!(runs > 400 && runs < 600, "Runs test failed: {}", runs);
    }

    #[test]
    #[cfg(feature = "fast_rng")]
    fn test_serial_correlation() {
        // Serial correlation test
        let mut rng = Xoshiro256Plus::new(42);
        let mut values = Vec::new();

        // Generate 1000 values
        for _ in 0..1000 {
            values.push(rng.next_u64());
        }

        // Calculate correlation between consecutive values
        let mut covariance = 0.0;
        let mut variance = 0.0;
        let mean: f64 = values.iter().map(|&v| v as f64).sum::<f64>() / values.len() as f64;

        for i in 1..values.len() {
            let diff1 = values[i - 1] as f64 - mean;
            let diff2 = values[i] as f64 - mean;
            covariance += diff1 * diff2;
        }

        for &value in &values {
            let diff = value as f64 - mean;
            variance += diff * diff;
        }

        covariance /= (values.len() - 1) as f64;
        variance /= (values.len() - 1) as f64;

        let correlation = if variance > 0.0 {
            covariance / variance
        } else {
            0.0
        };

        // Correlation coefficient should be close to 0 for good RNG (|r| < 0.1 is reasonable)
        assert!(
            correlation.abs() < 0.1,
            "Serial correlation too high: {}",
            correlation
        );
    }

    #[test]
    #[cfg(feature = "fast_rng")]
    fn test_birthday_spacing() {
        // Birthday spacing test: check for collisions
        let mut rng = Xoshiro256Plus::new(42);
        let mut seen = std::collections::HashSet::new();
        let mut collisions = 0;

        // Generate 10000 values and check for collisions
        for _ in 0..10000 {
            let value = rng.next_u64();
            if !seen.insert(value) {
                collisions += 1;
            }
        }

        // For good RNG, collisions should be very rare
        assert!(collisions < 10, "Too many collisions: {}", collisions);
    }
}
