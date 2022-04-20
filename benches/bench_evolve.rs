use criterion::{criterion_group, criterion_main, Criterion};
use spea2::mocks;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("spea2", |b| {
        b.iter(|| spea2::evolve(mocks::get_spea2model()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
