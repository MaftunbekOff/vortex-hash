use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use vortex_hash::hardware::UltraPerformance;

fn bench_hash_small(c: &mut Criterion) {
    let data = vec![0u8; 1024]; // 1KB
    c.bench_function("hash_ultra_optimized_small", |b| {
        b.iter(|| UltraPerformance::hash_ultra_optimized(black_box(&data)))
    });
}

fn bench_hash_medium(c: &mut Criterion) {
    let data = vec![0u8; 1024 * 1024]; // 1MB
    c.bench_function("hash_ultra_optimized_medium", |b| {
        b.iter(|| UltraPerformance::hash_ultra_optimized(black_box(&data)))
    });
}

fn bench_hash_large(c: &mut Criterion) {
    let data = vec![0u8; 10 * 1024 * 1024]; // 10MB
    c.bench_function("hash_ultra_optimized_large", |b| {
        b.iter(|| UltraPerformance::hash_ultra_optimized(black_box(&data)))
    });
}

fn bench_hash_parallel(c: &mut Criterion) {
    let data = vec![0u8; 1024 * 1024]; // 1MB
    let n_threads = rayon::current_num_threads();
    let chunks: Vec<_> = (0..n_threads).map(|_| data.clone()).collect();

    c.bench_function("hash_ultra_optimized_parallel", |b| {
        b.iter(|| {
            chunks
                .par_iter()
                .map(|chunk| UltraPerformance::hash_ultra_optimized(black_box(chunk)))
                .collect::<Vec<_>>()
        })
    });
}

criterion_group!(
    benches,
    bench_hash_small,
    bench_hash_medium,
    bench_hash_large,
    bench_hash_parallel
);
criterion_main!(benches);
