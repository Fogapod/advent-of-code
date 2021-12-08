use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d10/full").unwrap();

    c.bench_function("d10-1", |b| b.iter(|| d10::run1(input)));
    c.bench_function("d10-2", |b| b.iter(|| d10::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
