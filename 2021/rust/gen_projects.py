import subprocess

from pathlib import Path


for i in range(9, 25 + 1):
    day_str = f"d{i:0>2}"

    subprocess.run(["cargo", "new", day_str])

    with open(Path(day_str) / "src" / "main.rs", "w") as main:
        main.write(
            f"""fn main() {{
    let input = &std::fs::read(format!("../input/{{}}/full", module_path!())).unwrap();

    println!("{day_str}::run1       {{}}", {day_str}::run1(input));
    println!("{day_str}::run2       {{}}", {day_str}::run2(input));
}}"""
        )

    with open(Path(day_str) / "src" / "lib.rs", "w") as lib:
        lib.write(
            """pub fn run1(input: &[u8]) -> i64 {
    input.len() as i64
}

pub fn run2(input: &[u8]) -> i64 {
    input.len() as i64
}"""
        )

    with open(Path("benches") / f"{day_str}.rs", "w") as bench:
        bench.write(
            f"""use criterion::*;

fn bench(c: &mut Criterion) {{
    let input = &std::fs::read("../input/{day_str}/full").unwrap();

    c.bench_function("{day_str}-1", |b| b.iter(|| {day_str}::run1(input)));
    c.bench_function("{day_str}-2", |b| b.iter(|| {day_str}::run2(input)));
}}

criterion_group!(benches, bench);
criterion_main!(benches);
"""
        )

    print(f'{day_str} = {{ path = "{day_str}" }}')

    with open("Cargo.toml", "a") as f:
        f.write(
            f"""
[[bin]]
name = "{day_str}"
path = "{day_str}/src/main.rs"

[[bench]]
name = "{day_str}"
harness = false
"""
        )
