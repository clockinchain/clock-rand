//! Performance comparison benchmarks against rand crate

use clock_rand::*;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg64 as RandPcg64;
use rand_xoshiro::Xoshiro256Plus as RandXoshiro256Plus;
use rand_chacha::ChaCha20Rng as RandChaCha20Rng;

fn bench_fast_rng_comparison(c: &mut Criterion) {
    let mut clock_xoshiro = Xoshiro256Plus::new(12345);
    let mut rand_xoshiro = RandXoshiro256Plus::from_seed([42u8; 32]);

    let mut group = c.benchmark_group("fast_rng_u64");

    group.bench_function("clock_rand_xoshiro256", |b| {
        b.iter(|| std::hint::black_box(<Xoshiro256Plus as Rng>::next_u64(&mut clock_xoshiro)))
    });

    group.bench_function("rand_xoshiro256", |b| {
        b.iter(|| std::hint::black_box(rand_xoshiro.next_u64()))
    });

    group.finish();

    let mut clock_pcg = Pcg64::new(12345);
    let mut rand_pcg = RandPcg64::from_seed([42u8; 32]);

    let mut group = c.benchmark_group("fast_rng_pcg_u64");

    group.bench_function("clock_rand_pcg64", |b| {
        b.iter(|| std::hint::black_box(<Pcg64 as Rng>::next_u64(&mut clock_pcg)))
    });

    group.bench_function("rand_pcg64", |b| {
        b.iter(|| std::hint::black_box(rand_pcg.next_u64()))
    });

    group.finish();
}

fn bench_fill_bytes_comparison(c: &mut Criterion) {
    let mut clock_xoshiro = Xoshiro256Plus::new(12345);
    let mut rand_xoshiro = RandXoshiro256Plus::from_seed([42u8; 32]);

    let mut group = c.benchmark_group("fill_bytes_64");

    group.bench_function("clock_rand_xoshiro256", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| clock_xoshiro.fill_bytes(std::hint::black_box(&mut buf)))
    });

    group.bench_function("rand_xoshiro256", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rand_xoshiro.fill_bytes(std::hint::black_box(&mut buf)))
    });

    group.finish();

    let mut group = c.benchmark_group("fill_bytes_1024");

    group.bench_function("clock_rand_xoshiro256", |b| {
        let mut buf = [0u8; 1024];
        b.iter(|| clock_xoshiro.fill_bytes(std::hint::black_box(&mut buf)))
    });

    group.bench_function("rand_xoshiro256", |b| {
        let mut buf = [0u8; 1024];
        b.iter(|| rand_xoshiro.fill_bytes(std::hint::black_box(&mut buf)))
    });

    group.finish();
}

#[cfg(feature = "crypto_rng")]
fn bench_crypto_rng_comparison(c: &mut Criterion) {
    let seed = Seed::from_bytes(b"benchmark_seed_1234567890123456".to_vec()).unwrap();
    let mut clock_chacha = ChaCha20Rng::new(&seed).unwrap();
    let mut rand_chacha =
        RandChaCha20Rng::from_seed(seed.as_ref()[..32].try_into().unwrap());

    let mut group = c.benchmark_group("crypto_rng_u64");

    group.bench_function("clock_rand_chacha20", |b| {
        b.iter(|| std::hint::black_box(<ChaCha20Rng as Rng>::next_u64(&mut clock_chacha)))
    });

    group.bench_function("rand_chacha20", |b| {
        b.iter(|| std::hint::black_box(rand_chacha.next_u64()))
    });

    group.finish();
}

#[cfg(feature = "crypto_rng")]
criterion_group!(
    comparison_benches,
    bench_fast_rng_comparison,
    bench_fill_bytes_comparison,
    bench_crypto_rng_comparison
);

#[cfg(not(feature = "crypto_rng"))]
criterion_group!(
    comparison_benches,
    bench_fast_rng_comparison,
    bench_fill_bytes_comparison
);

criterion_main!(comparison_benches);
