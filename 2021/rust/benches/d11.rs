use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d11/full").unwrap();

    c.bench_function("d11-1", |b| b.iter(|| d11::run1(input)));
    c.bench_function("d11-2", |b| b.iter(|| d11::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
