use aoc_2025_06::{file_parser::FileParserTrait, solve, solve_pt1, solve_pt2};
use criterion::{Criterion, criterion_group, criterion_main};

use aoc_2025_06::file_parser::FileParser;

fn bench_solve(c: &mut Criterion) {
    let my_file = FileParser::new("data/input.txt");
    let my_data = my_file.get_str();
    c.bench_function("Combined", move |b| b.iter(|| solve(my_data)));
}

fn bench_pt1(c: &mut Criterion) {
    let my_file = FileParser::new("data/input.txt");
    let my_data = my_file.get_str();
    c.bench_function("Part 1", move |b| b.iter(|| solve_pt1(my_data)));
}

fn bench_pt2(c: &mut Criterion) {
    let my_file = FileParser::new("data/input.txt");
    let my_data = my_file.get_str();
    c.bench_function("Part 2", move |b| b.iter(|| solve_pt2(my_data)));
}

criterion_group! {
name = benches;
config= Criterion::default();
targets= bench_solve, bench_pt1, bench_pt2
}
criterion_main!(benches);
