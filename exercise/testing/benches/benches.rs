#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn trial_benchmark(c: &mut Criterion) {
    c.bench_function("splish", |b| b.iter(|| sploosh(8, 9, 10)));
}

criterion_group!(benches, trial_benchmark);
criterion_main!(benches);
