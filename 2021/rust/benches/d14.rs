use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d14/full").unwrap();

    c.bench_function("d14-1", |b| b.iter(|| d14::run1(input)));
    c.bench_function("d14-2", |b| b.iter(|| d14::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
