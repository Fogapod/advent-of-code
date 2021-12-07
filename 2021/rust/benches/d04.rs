use d04;

use criterion::*;

fn bench(c: &mut Criterion) {
    let input = &std::fs::read("../input/d04/full").unwrap();

    c.bench_function("d04-1", |b| b.iter(|| d04::run1(input)));
    c.bench_function("d04-2", |b| b.iter(|| d04::run2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
