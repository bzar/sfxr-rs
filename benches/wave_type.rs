use criterion::{criterion_group, criterion_main, Criterion};
use sfxr::{Generator, Sample, WaveType};

fn criterion_benchmark(c: &mut Criterion) {
    // Defaults with simple wave types

    c.bench_function("sine wave", |b| {
        let mut buffer = [0.0; 44_100];

        let mut sample = Sample::new();
        sample.wave_type = WaveType::Sine;
        let mut generator = Generator::new(sample);

        b.iter(|| {
            generator.generate(&mut buffer);
        });
    });
    c.bench_function("square wave", |b| {
        let mut buffer = [0.0; 44_100];

        let mut sample = Sample::new();
        sample.wave_type = WaveType::Square;
        let mut generator = Generator::new(sample);

        b.iter(|| {
            generator.generate(&mut buffer);
        });
    });
    c.bench_function("triangle wave", |b| {
        let mut buffer = [0.0; 44_100];

        let mut sample = Sample::new();
        sample.wave_type = WaveType::Triangle;
        let mut generator = Generator::new(sample);

        b.iter(|| {
            generator.generate(&mut buffer);
        });
    });
    c.bench_function("noise wave", |b| {
        let mut buffer = [0.0; 44_100];

        let mut sample = Sample::new();
        sample.wave_type = WaveType::Noise;
        let mut generator = Generator::new(sample);

        b.iter(|| {
            generator.generate(&mut buffer);
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = criterion_benchmark
}
criterion_main!(benches);
