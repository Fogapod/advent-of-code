use d05;

use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read_to_string("../input/d05/full").unwrap();

    c.bench_function("d05-1", |b| b.iter(|| d05::run1(input)));
    c.bench_function("d05-2", |b| b.iter(|| d05::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
