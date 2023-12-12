use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn something(c: &mut Criterion) {
    c.bench_function("Something1", |b| {
        // Something1 usage
        // wrap value `black_box` to disable/avoid compiler optimizations
    });

    c.bench_function("Something2", |b| {
        // Something2 usage
        // wrap value `black_box` to disable/avoid compiler optimizations
    });

    c.bench_function("Something3", |b| {
        // Something3 usage
        // wrap value `black_box` to disable/avoid compiler optimizations
    });
}

criterion_group!(benches, something);
criterion_main!(benches);
