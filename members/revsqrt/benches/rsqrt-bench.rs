use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use revsqrt::{fast_inverse_sqrt, regular_inverse_sqrt};

const SIZE: f32 = 1337.1337;
const FCONST: f32 = 1024.12481224;
const FCONST1: f32 = 4025.724812234;

pub fn single_input(c: &mut Criterion) {
    c.bench_with_input(BenchmarkId::new("regular rsqrt", SIZE), &SIZE, |b, &s| {
        b.iter(|| regular_inverse_sqrt(s))
    });
    c.bench_with_input(BenchmarkId::new("fast rsqrt", SIZE), &SIZE, |b, &s| {
        b.iter(|| fast_inverse_sqrt(s))
    });
}

pub fn multi_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_input");
    for size in [
        FCONST,
        2.2 * FCONST,
        4.24 * FCONST,
        8.64 * FCONST,
        16.12 * FCONST,
    ]
    .iter()
    {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("regular rsqrt mixed input", FCONST),
            size,
            |b, &size| {
                b.iter(|| regular_inverse_sqrt(size));
            },
        );
    }
    for size in [
        FCONST1,
        2.2 * FCONST1,
        4.24 * FCONST1,
        8.64 * FCONST1,
        16.12 * FCONST1,
    ]
    .iter()
    {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("fast rsqrt mixed input", FCONST1),
            size,
            |b, &size| {
                b.iter(|| fast_inverse_sqrt(size));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, single_input, multi_input);
criterion_main!(benches);
