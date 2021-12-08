use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d18/full").unwrap();

    c.bench_function("d18-1", |b| b.iter(|| d18::run1(input)));
    c.bench_function("d18-2", |b| b.iter(|| d18::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
