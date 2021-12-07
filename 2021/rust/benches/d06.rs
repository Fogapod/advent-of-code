use d06;

use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d06/full").unwrap();

    c.bench_function("d06-1", |b| b.iter(|| d06::run1(input)));
    c.bench_function("d06-1-simd", |b| b.iter(|| d06::run1_simd(input)));
    c.bench_function("d06-2", |b| b.iter(|| d06::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
