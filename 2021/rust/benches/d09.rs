use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d09/full").unwrap();

    c.bench_function("d09-1", |b| b.iter(|| d09::run1(input)));
    c.bench_function("d09-2", |b| b.iter(|| d09::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
