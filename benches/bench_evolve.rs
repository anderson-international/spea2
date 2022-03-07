use criterion::{criterion_group, criterion_main, Criterion};
use spea2::{self};

fn criterion_benchmark(c: &mut Criterion) {
    let population = spea2::initialise_population();
    c.bench_function("spea2 evolve 20", |b| b.iter(|| spea2::evolve(&population)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
