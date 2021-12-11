use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d01/full").unwrap();

    c.bench_function("d01-1", |b| b.iter(|| d01::run1(input)));
    c.bench_function("d01-2", |b| b.iter(|| d01::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
