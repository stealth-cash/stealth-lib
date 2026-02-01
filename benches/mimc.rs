//! Benchmarks for MiMC hash function.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stealth_lib::hash::MimcHasher;

fn bench_mimc_hash(c: &mut Criterion) {
    let hasher = MimcHasher::default();

    c.bench_function("mimc_hash", |b| {
        b.iter(|| hasher.hash(black_box(123456789), black_box(987654321)))
    });
}

fn bench_mimc_hash_single(c: &mut Criterion) {
    let hasher = MimcHasher::default();

    c.bench_function("mimc_hash_single", |b| {
        b.iter(|| hasher.hash_single(black_box(123456789)))
    });
}

fn bench_mimc_sponge(c: &mut Criterion) {
    let hasher = MimcHasher::default();
    let key = hasher.field_prime();

    c.bench_function("mimc_sponge", |b| {
        b.iter(|| hasher.mimc_sponge(black_box(123), black_box(456), black_box(key)))
    });
}

criterion_group!(benches, bench_mimc_hash, bench_mimc_hash_single, bench_mimc_sponge);
criterion_main!(benches);
