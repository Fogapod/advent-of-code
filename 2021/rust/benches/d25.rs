use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d25/full").unwrap();

    c.bench_function("d25-1", |b| b.iter(|| d25::run1(input)));
    c.bench_function("d25-2", |b| b.iter(|| d25::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
