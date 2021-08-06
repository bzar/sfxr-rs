use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sfxr::Sample;

fn criterion_benchmark(c: &mut Criterion) {
    // Default samples

    c.bench_function("pickup", |b| {
        b.iter(|| {
            black_box(Sample::pickup(None));
        });
    });
    c.bench_function("laser", |b| {
        b.iter(|| {
            black_box(Sample::laser(None));
        });
    });
    c.bench_function("explosion", |b| {
        b.iter(|| {
            black_box(Sample::explosion(None));
        });
    });
    c.bench_function("powerup", |b| {
        b.iter(|| {
            black_box(Sample::powerup(None));
        });
    });
    c.bench_function("hit", |b| {
        b.iter(|| {
            black_box(Sample::hit(None));
        });
    });
    c.bench_function("jump", |b| {
        b.iter(|| {
            black_box(Sample::jump(None));
        });
    });
    c.bench_function("blip", |b| {
        b.iter(|| {
            black_box(Sample::blip(None));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
