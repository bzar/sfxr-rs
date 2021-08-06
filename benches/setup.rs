use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sfxr::{Generator, Sample, WaveType};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("setup without generate", |b| {
        b.iter(|| {
            let mut sample = Sample::new();
            sample.wave_type = WaveType::Sine;
            // Black box will make sure the compiler won't compile away the unused results
            let _generator = black_box(Generator::new(sample));
        });
    });

    c.bench_function("reset", |b| {
        let mut buffer = [0.0; 44_100];

        let mut sample = Sample::new();
        sample.wave_type = WaveType::Sine;
        let mut generator = Generator::new(sample);

        generator.generate(&mut buffer);

        b.iter(|| {
            generator.reset();
        });
    });

    c.bench_function("mutate", |b| {
        let mut sample = Sample::new();

        b.iter(|| {
            sample.mutate(None);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
