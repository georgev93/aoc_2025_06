use aoc_2025_06::{solve, solve_pt1, solve_pt2};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_solve(c: &mut Criterion) {
    c.bench_function("Combined", |b| b.iter(|| solve("data/input.txt")));
}

fn bench_pt1(c: &mut Criterion) {
    c.bench_function("Part 1", |b| b.iter(|| solve_pt1("data/input.txt")));
}

fn bench_pt2(c: &mut Criterion) {
    c.bench_function("Part 2", |b| b.iter(|| solve_pt2("data/input.txt")));
}

criterion_group! {
name = benches;
config= Criterion::default();
targets= bench_solve, bench_pt1, bench_pt2
}
criterion_main!(benches);
