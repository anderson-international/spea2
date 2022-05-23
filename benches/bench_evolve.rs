use criterion::{criterion_group, criterion_main, Criterion};
use spea2::{
    mocks::{self, MockModel},
    EA,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("spea2", |b| {
        b.iter(|| {
            let mut model: MockModel = mocks::get_model();
            let mut ea = EA::new(&mut model);
            spea2::evolve(&mut ea, &model)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
