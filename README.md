# clock-rand

[![crates.io](https://img.shields.io/crates/v/clock-rand.svg)](https://crates.io/crates/clock-rand)
[![Documentation](https://docs.rs/clock-rand/badge.svg)](https://docs.rs/clock-rand)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/Olyntar-Labs/clock-rand/blob/main/LICENSE)
[![CI](https://github.com/Olyntar-Labs/clock-rand/workflows/CI/badge.svg)](https://github.com/Olyntar-Labs/clock-rand/actions)
[![Code Coverage](https://codecov.io/gh/Olyntar-Labs/clock-rand/branch/main/graph/badge.svg)](https://codecov.io/gh/Olyntar-Labs/clock-rand)
[![Security Audit](https://github.com/Olyntar-Labs/clock-rand/workflows/Security%20Scan/badge.svg)](https://github.com/Olyntar-Labs/clock-rand/actions)

Custom blockchain-aware RNG crate with fast and crypto-secure RNGs by Olyntar Labs, an Olyntar company.

## Features

- **Fast RNGs**: Xoshiro256+, PCG64 for high-performance simulations
- **Crypto RNGs**: Blake3-DRBG, ChaCha20-based RNGs for security-critical operations
- **Custom RNGs**: ChainSeed-X, EntroCrypt for blockchain-specific use cases
- **Blockchain-aware**: Native support for block hash, timestamp, and VRF seeding
- **Fork detection**: Automatic reseeding on blockchain forks
- **no_std compatible**: Works in embedded and WASM environments
- **Thread-safe**: Optional thread-safe wrappers for multi-threaded applications

## Quick Start

```rust
use clock_rand::*;

// Fast RNG for simulations
let mut rng = Xoshiro256Plus::new(42);
let value: u64 = rng.next_u64();

// Crypto RNG for security
let seed = Seed::from_block_hash(&[0x42u8; 32]).unwrap();
let mut crypto_rng = Blake3Drbg::new(&seed).unwrap();
let key: [u8; 32] = {
    let mut k = [0u8; 32];
    crypto_rng.fill_bytes(&mut k);
    k
};

// Blockchain-aware RNG with fork detection
let mut chain_rng = ChainSeedX::builder()
    .with_block_hash([0x01u8; 32])
    .with_timestamp(12345)
    .with_fork_detection(true)
    .build()
    .unwrap();

// Check for fork and reseed if needed
let fork_detected = chain_rng.check_fork(&[0x02u8; 32]).unwrap();
```

## Security

**IMPORTANT**: Use the correct RNG for your use case:

- **Fast RNGs** (Xoshiro256+, PCG64): Use for simulations, games, non-security applications
- **Crypto RNGs** (Blake3Drbg, ChaCha20Rng): Use for key generation, signatures, security-critical operations
- **Custom RNGs** (ChainSeed-X, EntroCrypt): Use for blockchain-specific randomness with fork detection

See [SECURITY.md](docs/SECURITY.md) for detailed security considerations.

## Performance

| RNG Type | Throughput | Use Case |
|----------|-----------|----------|
| Xoshiro256+ | ~2 GB/s | Fast simulations |
| PCG64 | ~1.5 GB/s | General purpose |
| Blake3Drbg | ~500 MB/s | Crypto operations |
| ChainSeed-X | ~300 MB/s | Blockchain apps |

See [PERFORMANCE.md](docs/PERFORMANCE.md) for detailed benchmarks.

## Migration from `rand`

clock-rand is designed to be compatible with the `rand` crate ecosystem:

```rust
// Old (rand)
use rand::RngCore;
let mut rng = rand::thread_rng();
let value = rng.next_u64();

// New (clock-rand)
use clock_rand::Rng;
let mut rng = clock_rand::Xoshiro256Plus::new(42);
let value = rng.next_u64();
```

## Features

- `fast_rng` - Fast deterministic RNGs (default)
- `crypto_rng` - Cryptographically secure RNGs
- `custom_rng` - Custom blockchain RNGs
- `thread_safe` - Thread-safe wrappers
- `fork_safe` - Fork detection and reseeding
- `serde` - Serialization support
- `security` - Security features (zeroize)
- `wasm` - WASM bindings
- `std` - Standard library features (default)

## Examples

See the [examples/](examples/) directory for:
- Basic usage
- Blockchain seeding
- Fork detection
- Thread-safe usage
- Serialization
- WASM usage

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Detailed architecture
- [SECURITY.md](docs/SECURITY.md) - Security considerations
- [PERFORMANCE.md](docs/PERFORMANCE.md) - Performance characteristics

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Security

If you discover a security issue, please report it privately to [security@olyntar.com](mailto:security@olyntar.com) instead of opening an issue.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
