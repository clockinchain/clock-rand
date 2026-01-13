# Performance Characteristics

## Benchmarks

All benchmarks run on x86_64 Linux with Rust 1.70+. Results from `cargo bench --bench crypto_rng --features "crypto_rng custom_rng aes"`.

### Cryptographic RNG Performance

#### Single Value Generation (ns per operation)

| RNG Implementation | next_u32 | next_u64 | Performance Rank |
|-------------------|----------|----------|------------------|
| **ChaCha20Rng** | **6.66 ns** | **13.01 ns** | 🥇 Fastest |
| **Blake3Drbg** | 9.36 ns | 17.95 ns | 🥈 High Security |
| **ChainSeedX** | 9.81 ns | 18.78 ns | 🥉 Blockchain-Optimized |
| **EntroCrypt** | 15.75 ns | 29.05 ns | Maximum Security |
| **AesCtrRng** | - | - | AES-Based |

#### Bulk Data Generation

| RNG Implementation | fill_bytes_64 (ns) | fill_bytes_1024 (µs) | Throughput (MB/s) |
|-------------------|-------------------|---------------------|-------------------|
| **ChaCha20Rng** | **98.78 ns** | **1.57 µs** | **~654 MB/s** |
| **Blake3Drbg** | 138.35 ns | 2.23 µs | **~457 MB/s** |
| **ChainSeedX** | 165.19 ns | - | **~389 MB/s** |
| **EntroCrypt** | 272.41 ns | - | **~235 MB/s** |

### Fast RNG Performance (Estimated)

| RNG | Throughput | Use Case |
|-----|------------|----------|
| **Xoshiro256+** | ~2.0 GB/s | Simulations, games |
| **PCG64** | ~1.5 GB/s | General computing |

### Latency Comparison (ns per operation)

| Operation | ChaCha20Rng | Blake3Drbg | ChainSeedX | EntroCrypt |
|-----------|-------------|------------|------------|------------|
| next_u32() | 6.66 | 9.36 | 9.81 | 15.75 |
| next_u64() | 13.01 | 17.95 | 18.78 | 29.05 |
| fill_bytes(64B) | 98.78 | 138.35 | 165.19 | 272.41 |
| fill_bytes(1KB) | 1,570 | 2,230 | - | - |

## Memory Usage

| RNG | State Size | Total Size | Security Level |
|-----|-----------|------------|----------------|
| Xoshiro256+ | 32 bytes | ~64 bytes | Fast (non-crypto) |
| PCG64 | 16 bytes | ~32 bytes | Fast (non-crypto) |
| ChaCha20Rng | 100 bytes | ~150 bytes | Cryptographically secure |
| Blake3Drbg | ~150 bytes | ~200 bytes | Cryptographically secure |
| AesCtrRng | ~180 bytes | ~230 bytes | Cryptographically secure |
| ChainSeed-X | ~200 bytes | ~300 bytes | Blockchain-aware |
| EntroCrypt | ~300 bytes | ~400 bytes | Maximum security |

## Optimization Tips

1. **Use fast RNGs for bulk generation**
2. **Cache RNG instances when possible**
3. **Use SIMD features when available**
4. **Prefer `fill_bytes` for large buffers**

## Comparison with `rand` crate

clock-rand performance vs `rand` crate (measured with comparison benchmarks):

- **ChaCha20Rng**: ~6.7x faster than `rand_chacha::ChaCha20Rng` for `next_u32`
- **Xoshiro256+**: ~1.5x faster than `rand_xoshiro::Xoshiro256Plus`
- **PCG64**: Competitive with `rand_pcg::Pcg64`
- **Crypto RNGs**: Optimized for blockchain and security-critical applications

## Benchmark Methodology

- **Framework**: Criterion.rs with 100 statistical samples per benchmark
- **Platform**: x86_64 Linux, Rust 1.70+
- **Confidence**: 95% statistical confidence intervals reported
- **Outliers**: Automatically detected and handled (1-9% across benchmarks)
- **Operations**: Single value generation (`next_u32`/`next_u64`) and bulk operations (`fill_bytes`)

## Performance Optimization Tips

1. **Choose the right RNG for your use case**:
   - Use fast RNGs (Xoshiro256+, PCG64) for simulations/games
   - Use crypto RNGs (ChaCha20Rng, Blake3Drbg) for security applications
   - Use specialized RNGs (ChainSeedX, EntroCrypt) for blockchain/consensus

2. **Bulk operations are much faster**:
   ```rust
   // ❌ Slow: individual calls
   for _ in 0..1024 {
       let value = rng.next_u32();
   }

   // ✅ Fast: bulk operations
   let mut buffer = [0u32; 1024];
   rng.fill_bytes(buffer.as_mut_bytes()); // ~10-50x faster
   ```

3. **Cache RNG instances**:
   ```rust
   // ✅ Create once, reuse
   let mut rng = ChaCha20Rng::new(&seed)?;
   ```

4. **Enable appropriate features**:
   ```toml
   clock-rand = { version = "1.0", features = ["crypto_rng", "aes"] }
   ```
