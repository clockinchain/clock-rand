# Security Considerations

## RNG Selection

### Fast RNGs (Xoshiro256+, PCG64)

**DO NOT USE** for:
- Key generation
- Nonce generation
- Signature operations
- Any security-critical operations

**USE FOR**:
- Simulations
- Games
- Testing
- Non-security applications

### Crypto RNGs (Blake3Drbg, ChaCha20Rng)

**USE FOR**:
- Key generation
- Nonce generation
- Signature operations
- Security-critical randomness

**Guarantees**:
- Cryptographically secure
- Suitable for cryptographic operations
- Pass statistical tests

### Custom RNGs (ChainSeed-X, EntroCrypt)

**USE FOR**:
- Blockchain applications
- Fork-aware randomness
- Deterministic consensus operations

**Features**:
- Fork detection and automatic reseeding
- Blockchain state seeding
- Deterministic replay

## Seed Management

### Weak Seeds

**NEVER USE**:
- All zeros
- Predictable sequences
- Publicly known values

**ALWAYS USE**:
- Cryptographically secure entropy sources
- Block hashes (for blockchain apps)
- VRF outputs (for verifiable randomness)

### Seed Validation

clock-rand automatically validates seeds:
- Rejects all-zero seeds
- Validates seed size
- Checks for weak patterns

## Constant-Time Operations

Crypto RNGs use constant-time operations where applicable to prevent timing attacks.

## Zeroization

When the `security` feature is enabled, sensitive state is automatically zeroized on drop.

## Recommendations

1. **Always use crypto RNGs for security-critical operations**
2. **Validate seeds before use**
3. **Use fork detection for blockchain applications**
4. **Enable security features in production**
5. **Regular security audits**

## Reporting Security Issues

Please report security issues privately to the maintainers.
