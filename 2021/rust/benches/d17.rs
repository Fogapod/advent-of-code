use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d17/full").unwrap();

    c.bench_function("d17-1", |b| b.iter(|| d17::run1(input)));
    c.bench_function("d17-2", |b| b.iter(|| d17::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
