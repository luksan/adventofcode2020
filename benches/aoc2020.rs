use adventofcode2020 as aoc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let d = aoc::day9::load_input();
    c.bench_function("day 9 part 1", |b| b.iter(|| aoc::day9::part1(&d, 25)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
