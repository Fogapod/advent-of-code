use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d02/full").unwrap();

    c.bench_function("d08-1", |b| b.iter(|| d08::run1(input)));
    c.bench_function("d08-2", |b| b.iter(|| d08::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
