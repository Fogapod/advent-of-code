use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d22/full").unwrap();

    c.bench_function("d22-1", |b| b.iter(|| d22::run1(input)));
    c.bench_function("d22-2", |b| b.iter(|| d22::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
