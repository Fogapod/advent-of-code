use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d16/full").unwrap();

    c.bench_function("d16-1", |b| b.iter(|| d16::run1(input)));
    c.bench_function("d16-2", |b| b.iter(|| d16::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
