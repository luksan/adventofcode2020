use adventofcode2020 as aoc;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    use aoc::y2021::day14::{bench, bench_input};
    let d = bench_input();
    c.bench_function("2021 day 14", |b| b.iter(|| bench(&d)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
