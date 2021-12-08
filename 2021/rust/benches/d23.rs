use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d23/full").unwrap();

    c.bench_function("d23-1", |b| b.iter(|| d23::run1(input)));
    c.bench_function("d23-2", |b| b.iter(|| d23::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
