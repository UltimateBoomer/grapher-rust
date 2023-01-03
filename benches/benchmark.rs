use criterion::{criterion_group, criterion_main, Criterion};
use grapher::*;

pub fn bench(c: &mut Criterion) {
    c.bench_function("ZeroGrapher draw_2d", |b| {
        b.iter(|| ZeroGrapher.draw_2d((500, 500), (-1.0f32, -1.0, 1.0, 1.0)))
    });
    c.bench_function("DistToGrapher draw_2d", |b| {
        b.iter(|| DistToGrapher.draw_2d((500, 500), (-1.0, -1.0, 1.0, 1.0)))
    });
    c.bench_function("AddGrapher draw_2d", |b| {
        b.iter(|| AddGrapher.draw_2d((500, 500), (-1.0, -1.0, 1.0, 1.0)))
    });
    c.bench_function("MandelbrotGrapher draw_2d", |b| {
        b.iter(|| {
            MandelbrotGrapher {
                iterations: 10,
            }
            .draw_2d((500, 500), (-1.0, -1.0, 1.0, 1.0))
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
