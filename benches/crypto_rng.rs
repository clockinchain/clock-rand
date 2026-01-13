//! Performance benchmarks for crypto RNG implementations

use clock_rand::*;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_chacha20(c: &mut Criterion) {
    let seed = Seed::from_bytes(b"benchmark_seed_1234567890123456".to_vec()).unwrap();
    let mut rng = ChaCha20Rng::from_seed(seed);

    c.bench_function("chacha20_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("chacha20_next_u64", |b| {
        b.iter(|| std::hint::black_box(<ChaCha20Rng as Rng>::next_u64(&mut rng)))
    });

    c.bench_function("chacha20_fill_bytes_64", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });

    c.bench_function("chacha20_fill_bytes_1024", |b| {
        let mut buf = [0u8; 1024];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });
}

fn bench_blake3_drbg(c: &mut Criterion) {
    let seed = Seed::from_bytes(b"benchmark_seed_1234567890123456".to_vec()).unwrap();
    let mut rng = Blake3Drbg::from_seed(seed);

    c.bench_function("blake3_drbg_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("blake3_drbg_next_u64", |b| {
        b.iter(|| std::hint::black_box(<Blake3Drbg as Rng>::next_u64(&mut rng)))
    });

    c.bench_function("blake3_drbg_fill_bytes_64", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });

    c.bench_function("blake3_drbg_fill_bytes_1024", |b| {
        let mut buf = [0u8; 1024];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });
}

#[cfg(feature = "aes")]
fn bench_aes_ctr(c: &mut Criterion) {
    let seed = Seed::from_bytes(b"benchmark_seed_1234567890123456".to_vec()).unwrap();
    let mut rng = AesCtrDrbg::from_seed(seed);

    c.bench_function("aes_ctr_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("aes_ctr_next_u64", |b| {
        b.iter(|| std::hint::black_box(<AesCtrDrbg as Rng>::next_u64(&mut rng)))
    });

    c.bench_function("aes_ctr_fill_bytes_64", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });
}

fn bench_chainseed_x(c: &mut Criterion) {
    let mut rng = ChainSeedX::builder()
        .with_block_hash([0x42u8; 32])
        .with_timestamp(1234567890)
        .build()
        .unwrap();

    c.bench_function("chainseed_x_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("chainseed_x_next_u64", |b| {
        b.iter(|| std::hint::black_box(<ChainSeedX as Rng>::next_u64(&mut rng)))
    });

    c.bench_function("chainseed_x_fill_bytes_64", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });
}

fn bench_entrocrypt(c: &mut Criterion) {
    let mut rng = EntroCrypt::builder()
        .with_seed(Seed::from_bytes(b"benchmark_seed_1234567890123456".to_vec()).unwrap())
        .build()
        .unwrap();

    c.bench_function("entrocrypt_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("entrocrypt_next_u64", |b| {
        b.iter(|| std::hint::black_box(<EntroCrypt as Rng>::next_u64(&mut rng)))
    });

    c.bench_function("entrocrypt_fill_bytes_64", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });
}

#[cfg(feature = "aes")]
criterion_group!(
    crypto_rng_benches,
    bench_chacha20,
    bench_blake3_drbg,
    bench_aes_ctr,
    bench_chainseed_x,
    bench_entrocrypt
);

#[cfg(not(feature = "aes"))]
criterion_group!(
    crypto_rng_benches,
    bench_chacha20,
    bench_blake3_drbg,
    bench_chainseed_x,
    bench_entrocrypt
);

criterion_main!(crypto_rng_benches);
