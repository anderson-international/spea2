use criterion::{criterion_group, criterion_main, Criterion};
use spea2::sack::SackPool;

fn criterion_benchmark(c: &mut Criterion) {
    let sack_count = 10;
    let sack_max_weight = 50.0;
    let model = SackPool::new(sack_count, sack_max_weight);
    c.bench_function("spea2 evolve 20", |b| {
        b.iter(|| spea2::evolve(model.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
