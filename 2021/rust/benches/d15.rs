use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d15/full").unwrap();

    c.bench_function("d15-1", |b| b.iter(|| d15::run1(input)));
    c.bench_function("d15-2", |b| b.iter(|| d15::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
