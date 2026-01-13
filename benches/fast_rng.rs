//! Performance benchmarks for fast RNG implementations

use clock_rand::*;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_xoshiro256(c: &mut Criterion) {
    let mut rng = Xoshiro256Plus::new(12345);

    c.bench_function("xoshiro256_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("xoshiro256_next_u64", |b| {
        b.iter(|| std::hint::black_box(<Xoshiro256Plus as Rng>::next_u64(&mut rng)))
    });

    c.bench_function("xoshiro256_fill_bytes_64", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });

    c.bench_function("xoshiro256_fill_bytes_1024", |b| {
        let mut buf = [0u8; 1024];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });
}

fn bench_pcg64(c: &mut Criterion) {
    let mut rng = Pcg64::new(12345);

    c.bench_function("pcg64_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("pcg64_next_u64", |b| {
        b.iter(|| std::hint::black_box(<Pcg64 as Rng>::next_u64(&mut rng)))
    });

    c.bench_function("pcg64_fill_bytes_64", |b| {
        let mut buf = [0u8; 64];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });

    c.bench_function("pcg64_fill_bytes_1024", |b| {
        let mut buf = [0u8; 1024];
        b.iter(|| rng.fill_bytes(std::hint::black_box(&mut buf)))
    });
}

fn bench_splitmix64(c: &mut Criterion) {
    let mut rng = SplitMix64::new(12345);

    c.bench_function("splitmix64_next_u32", |b| {
        b.iter(|| std::hint::black_box(rng.next_u32()))
    });

    c.bench_function("splitmix64_next_u64", |b| {
        b.iter(|| std::hint::black_box(<SplitMix64 as Rng>::next_u64(&mut rng)))
    });
}

criterion_group!(
    fast_rng_benches,
    bench_xoshiro256,
    bench_pcg64,
    bench_splitmix64
);
criterion_main!(fast_rng_benches);
