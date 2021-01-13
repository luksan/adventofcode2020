use adventofcode2020 as aoc;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    use aoc::day23::{bench, bench_input};
    let d = bench_input();
    c.bench_function("day 23 part 2", |b| b.iter(|| bench(&d)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
