# Architecture

## Overview

clock-rand is organized into several modules:

- `fast/` - Fast deterministic RNGs
- `crypto/` - Cryptographically secure RNGs
- `custom/` - Custom blockchain-aware RNGs
- `distributions/` - Distribution traits and implementations
- `utils/` - Utility functions

## Design Principles

1. **no_std first**: Core functionality works without std
2. **Feature flags**: Enable only what you need
3. **Trait-based**: Compatible with rand ecosystem
4. **Security first**: Clear separation between fast and crypto RNGs
5. **Blockchain-aware**: Native support for blockchain state

## RNG Types

### Fast RNGs

- **Xoshiro256+**: Fast, high-quality, 256-bit state
- **PCG64**: Excellent statistical properties, small footprint
- **SplitMix64**: Ultra-fast seeding algorithm

### Crypto RNGs

- **Blake3Drbg**: Fast crypto RNG using Blake3
- **ChaCha20Rng**: ChaCha20-based crypto RNG
- **AesCtrRng**: FIPS-compliant AES-CTR DRBG

### Custom RNGs

- **ChainSeed-X**: Hybrid Blake3+PCG with fork detection
- **EntroCrypt**: Hybrid ChaCha20+Blake3 for maximum security
- **HashMix256**: Custom hash-based RNG

## Fork Detection

ChainSeed-X implements fork detection by:
1. Tracking recent block hashes
2. Comparing new block hash with expected
3. Automatically reseeding on divergence
4. Maintaining deterministic state

## Thread Safety

Thread-safe wrappers use:
- `Arc<Mutex<T>>` for exclusive access
- `Arc<RwLock<T>>` for read-write access
- Thread-local storage for global RNG

## Serialization

State serialization supports:
- Checkpointing for deterministic replay
- Versioned state format
- Cross-platform compatibility
