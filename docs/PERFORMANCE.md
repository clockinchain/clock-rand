# Performance Characteristics

## Benchmarks

All benchmarks run on x86_64 Linux with Rust 1.70+.

### Throughput (MB/s)

| RNG | u64/s | Bytes/s |
|-----|-------|---------|
| Xoshiro256+ | ~250M | ~2GB |
| PCG64 | ~190M | ~1.5GB |
| Blake3Drbg | ~60M | ~500MB |
| ChainSeed-X | ~40M | ~300MB |
| EntroCrypt | ~35M | ~280MB |

### Latency (ns per operation)

| Operation | Xoshiro256+ | PCG64 | Blake3Drbg |
|-----------|-------------|-------|------------|
| next_u64() | ~4ns | ~5ns | ~16ns |
| next_u32() | ~4ns | ~5ns | ~16ns |
| fill_bytes(1KB) | ~500ns | ~650ns | ~2μs |

## Memory Usage

| RNG | State Size | Total Size |
|-----|-----------|------------|
| Xoshiro256+ | 32 bytes | ~64 bytes |
| PCG64 | 16 bytes | ~32 bytes |
| Blake3Drbg | ~100 bytes | ~150 bytes |
| ChainSeed-X | ~200 bytes | ~300 bytes |

## Optimization Tips

1. **Use fast RNGs for bulk generation**
2. **Cache RNG instances when possible**
3. **Use SIMD features when available**
4. **Prefer `fill_bytes` for large buffers**

## Comparison with `rand` crate

clock-rand is designed to match or exceed `rand` performance:

- Xoshiro256+ is faster than `rand::rngs::SmallRng`
- PCG64 matches `rand::rngs::Pcg64` performance
- Crypto RNGs are optimized for blockchain use cases
