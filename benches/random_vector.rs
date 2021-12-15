use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracer::Vector;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("discard", |b| {
        b.iter(|| black_box(Vector::random_spherepoint()))
    });

    c.bench_function("gauss", |b| {
        b.iter(|| black_box(Vector::random_spherepoint_const()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
