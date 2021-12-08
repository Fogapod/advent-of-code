# Rust solutions

## WARNING
These solutions are optimized for speed and nothing else.  
For "correct" solutions, check out Python folder.

### General code structure
There is a main project in `rust` folder (this one) that can run all solutions and benchmark them.  
Each solution lives in an individual crate.

Each solution has `lib.rs` file with `run1` and `run2` functions, corresponding to part 1 and part 2 of solutions.

### Running solution

```sh
cargo run --bin d02
```

### Benchmarking solution

Benchmark certain day:

```sh
cargo bench --bench d02
```

Or benchmark individual part of day:

```sh
cargo bench --bench d02 d02-1
```
