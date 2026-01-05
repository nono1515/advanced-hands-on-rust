use criterion::{Criterion, criterion_group, criterion_main};
use my_librairy::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("random", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            rng.range(1.0f32..10_000_000f32);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
