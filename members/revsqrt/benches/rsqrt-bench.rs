#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use revsqrt::{inverse_sqrt, fast_inverse_sqrt};
const SIZE: f32 = 1337.1337;
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_with_input(BenchmarkId::new("regular rsqrt", SIZE),&SIZE, |b, &s| b.iter(|| inverse_sqrt(s)));
    c.bench_with_input(BenchmarkId::new("fast rsqrt", SIZE),&SIZE, |b, &s| b.iter(|| fast_inverse_sqrt(s)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
