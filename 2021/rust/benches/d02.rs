use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d02/full").unwrap();

    c.bench_function("d02-1", |b| b.iter(|| d02::run1(input)));
    c.bench_function("d02-2", |b| b.iter(|| d02::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
