use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d12/full").unwrap();

    c.bench_function("d12-1", |b| b.iter(|| d12::run1(input)));
    c.bench_function("d12-2", |b| b.iter(|| d12::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
