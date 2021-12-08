use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d19/full").unwrap();

    c.bench_function("d19-1", |b| b.iter(|| d19::run1(input)));
    c.bench_function("d19-2", |b| b.iter(|| d19::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
