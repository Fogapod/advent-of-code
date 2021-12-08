use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d20/full").unwrap();

    c.bench_function("d20-1", |b| b.iter(|| d20::run1(input)));
    c.bench_function("d20-2", |b| b.iter(|| d20::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
