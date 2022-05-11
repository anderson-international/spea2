use criterion::{criterion_group, criterion_main, Criterion};
use spea2::{mocks, model::Spea2Model};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("spea2", |b| {
        b.iter(|| {
            let spea2_model = &mut mocks::get_spea2model();
            let mut model = spea2_model.get_model();
            let mut mutation = spea2_model.get_mutation_operator();
            spea2::evolve(&mut model, &mut mutation)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
