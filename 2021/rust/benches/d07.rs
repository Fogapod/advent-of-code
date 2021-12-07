use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d07/full").unwrap();

    c.bench_function("d07-1", |b| b.iter(|| d07::run1(input)));
    c.bench_function("d07-1-brute", |b| b.iter(|| d07::run1_brute(input)));
    c.bench_function("d07-2", |b| b.iter(|| d07::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
