use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

pub mod file_parser;
use crate::file_parser::FileParser;

mod homework;
use crate::homework::Homework;

pub fn solve_pt1(input_file: &str) -> u64 {
    let my_homework = Homework::new(input_file);

    let mut total = 0u64;

    for i in 0..my_homework.num_of_problems {
        total += my_homework.solve_problem_human(i);
    }

    total
}

pub fn solve_pt2(input_file: &str) -> u64 {
    let my_homework = Homework::new(input_file);

    let mut total = 0u64;

    for i in 0..my_homework.num_of_problems {
        total += my_homework.solve_problem_ceph(i);
    }

    total
}

pub fn solve(input_file: &str) -> (u64, u64) {
    let my_homework = Homework::new(input_file);

    let mut total_pt1 = 0u64;
    let mut total_pt2 = 0u64;

    for i in 0..my_homework.num_of_problems {
        total_pt1 += my_homework.solve_problem_human(i);
        total_pt2 += my_homework.solve_problem_ceph(i);
    }

    (total_pt1, total_pt2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let my_file = FileParser::new("data/example.txt");
        let (part_1, part_2) = solve(my_file.get_str());
        assert_eq!(part_1, 4277556);
        assert_eq!(part_2, 3263827);
    }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 4277556);
        assert_eq!(solve_pt2(my_file.get_str()), 3263827);
    }

    #[test]
    fn actual() {
        let my_file = FileParser::new("data/input.txt");
        let (part_1, part_2) = solve(my_file.get_str());
        assert_eq!(part_1, 6757749566978);
        assert_eq!(part_2, 10603075273949);
    }

    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 6757749566978);
        assert_eq!(solve_pt2(my_file.get_str()), 10603075273949);
    }
}
