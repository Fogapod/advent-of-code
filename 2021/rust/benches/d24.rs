use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d24/full").unwrap();

    c.bench_function("d24-1", |b| b.iter(|| d24::run1(input)));
    c.bench_function("d24-2", |b| b.iter(|| d24::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
