use d6;

use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_bytes!("../input");

    c.bench_function("run1", |b| b.iter(|| d6::run1(INPUT)));
    c.bench_function("run2", |b| b.iter(|| d6::run2(INPUT)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
