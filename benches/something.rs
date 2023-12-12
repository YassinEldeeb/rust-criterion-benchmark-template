use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn something(c: &mut Criterion) {
    c.bench_function("Something1", |b| {
        b.iter(|| {
            // Something1 usage
            // wrap value `black_box` to disable/avoid compiler optimizations
            let x = i32::pow(2, 200);
            black_box(x);
        })
    });

    c.bench_function("Something2", |b| {
        b.iter(|| {
            // Something2 usage
            // wrap value `black_box` to disable/avoid compiler optimizations
            let x = i32::ilog(2, 40);
            black_box(x);
        })
    });

    c.bench_function("Something3", |b| {
        b.iter(|| {
            // Something3 usage
            // wrap value `black_box` to disable/avoid compiler optimizations
            let x = i32::ilog(9, 100);
            black_box(x);
        })
    });
}

criterion_group!(benches, something);
criterion_main!(benches);
