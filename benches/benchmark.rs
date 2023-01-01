use criterion::{criterion_group, criterion_main, Criterion};
use grapher::*;

pub fn bench(c: &mut Criterion) {
    c.bench_function("TrueGrapher draw_2d", |b| b.iter(|| TrueGrapher::draw_2d(500)));
    c.bench_function("CircleGrapher draw_2d", |b| b.iter(|| CircleGrapher::draw_2d(500)));
}

criterion_group!(benches, bench);
criterion_main!(benches);