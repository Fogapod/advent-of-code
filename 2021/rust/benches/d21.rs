use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d21/full").unwrap();

    c.bench_function("d21-1", |b| b.iter(|| d21::run1(input)));
    c.bench_function("d21-2", |b| b.iter(|| d21::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
