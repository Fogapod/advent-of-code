use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d13/full").unwrap();

    c.bench_function("d13-1", |b| b.iter(|| d13::run1(input)));
    c.bench_function("d13-2", |b| b.iter(|| d13::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
