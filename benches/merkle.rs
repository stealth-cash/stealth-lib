//! Benchmarks for Merkle tree operations.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use stealth_lib::MerkleTree;

fn bench_merkle_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_new");

    for levels in [10, 15, 20, 25].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(levels), levels, |b, &levels| {
            b.iter(|| MerkleTree::new(black_box(levels)).unwrap())
        });
    }

    group.finish();
}

fn bench_merkle_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_insert");

    for levels in [10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(levels), levels, |b, &levels| {
            b.iter_batched(
                || MerkleTree::new(levels).unwrap(),
                |mut tree| tree.insert(black_box(12345)),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn bench_merkle_insert_sequential(c: &mut Criterion) {
    c.bench_function("merkle_insert_100_leaves_depth_20", |b| {
        b.iter_batched(
            || MerkleTree::new(20).unwrap(),
            |mut tree| {
                for i in 0..100 {
                    tree.insert(black_box(i as u128)).unwrap();
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_merkle_prove(c: &mut Criterion) {
    let mut tree = MerkleTree::new(20).unwrap();
    for i in 0..100 {
        tree.insert(i as u128).unwrap();
    }

    c.bench_function("merkle_prove_depth_20", |b| {
        b.iter(|| tree.prove(black_box(50)).unwrap())
    });
}

fn bench_merkle_verify(c: &mut Criterion) {
    let mut tree = MerkleTree::new(20).unwrap();
    for i in 0..100 {
        tree.insert(i as u128).unwrap();
    }
    let proof = tree.prove(50).unwrap();
    let root = tree.root().unwrap();
    let hasher = tree.hasher();

    c.bench_function("merkle_verify_depth_20", |b| {
        b.iter(|| proof.verify(black_box(root), hasher))
    });
}

fn bench_is_known_root(c: &mut Criterion) {
    let mut tree = MerkleTree::new(20).unwrap();
    for i in 0..100 {
        tree.insert(i as u128).unwrap();
    }
    let root = tree.root().unwrap();

    c.bench_function("is_known_root_hit", |b| {
        b.iter(|| tree.is_known_root(black_box(root)))
    });

    c.bench_function("is_known_root_miss", |b| {
        b.iter(|| tree.is_known_root(black_box(99999)))
    });
}

criterion_group!(
    benches,
    bench_merkle_new,
    bench_merkle_insert,
    bench_merkle_insert_sequential,
    bench_merkle_prove,
    bench_merkle_verify,
    bench_is_known_root
);
criterion_main!(benches);
