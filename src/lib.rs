use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

mod file_parser;
use crate::file_parser::{FileParser, FileParserTrait};

mod homework;
use crate::homework::Homework;

pub fn solve_pt1(input_file: &str) -> u64 {
    let input_grid = FileParser::new(input_file).parse_grid();
    let my_homework = Arc::new(Homework::new(&input_grid));

    let total = Arc::new(AtomicU64::new(0));

    thread::scope(|s| {
        for i in 0..my_homework.num_of_problems {
            let total_clone = Arc::clone(&total);
            let my_homework_clone = Arc::clone(&my_homework);
            s.spawn(move || {
                total_clone.fetch_add(my_homework_clone.solve_problem_human(i), Ordering::SeqCst);
            });
        }
    });

    total.load(Ordering::Relaxed)
}

pub fn solve_pt2(input_file: &str) -> u64 {
    let input_grid = FileParser::new(input_file).parse_grid();
    let my_homework = Arc::new(Homework::new(&input_grid));

    let total = Arc::new(AtomicU64::new(0));

    thread::scope(|s| {
        for i in 0..my_homework.num_of_problems {
            let total_clone = Arc::clone(&total);
            let my_homework_clone = Arc::clone(&my_homework);
            s.spawn(move || {
                total_clone.fetch_add(my_homework_clone.solve_problem_ceph(i), Ordering::SeqCst);
            });
        }
    });

    total.load(Ordering::Relaxed)
}

pub fn solve(input_file: &str) -> (u64, u64) {
    let input_grid = FileParser::new(input_file).parse_grid();
    let my_homework = Arc::new(Homework::new(&input_grid));

    let total_pt1 = Arc::new(AtomicU64::new(0));
    let total_pt2 = Arc::new(AtomicU64::new(0));

    thread::scope(|s| {
        for i in 0..my_homework.num_of_problems {
            let total_pt1_clone = Arc::clone(&total_pt1);
            let total_pt2_clone = Arc::clone(&total_pt2);
            let my_homework_clone = Arc::clone(&my_homework);
            s.spawn(move || {
                total_pt1_clone
                    .fetch_add(my_homework_clone.solve_problem_human(i), Ordering::SeqCst);
                total_pt2_clone
                    .fetch_add(my_homework_clone.solve_problem_ceph(i), Ordering::SeqCst);
            });
        }
    });

    (
        total_pt1.load(Ordering::Relaxed),
        total_pt2.load(Ordering::Relaxed),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (part_1, part_2) = solve("data/example.txt");
        assert_eq!(part_1, 4277556);
        assert_eq!(part_2, 3263827);
    }

    #[test]
    fn example_pts() {
        assert_eq!(solve_pt1("data/example.txt"), 4277556);
        assert_eq!(solve_pt2("data/example.txt"), 3263827);
    }

    #[test]
    fn actual() {
        let (part_1, part_2) = solve("data/input.txt");
        assert_eq!(part_1, 6757749566978);
        assert_eq!(part_2, 10603075273949);
    }

    #[test]
    fn actual_pts() {
        assert_eq!(solve_pt1("data/input.txt"), 6757749566978);
        assert_eq!(solve_pt2("data/input.txt"), 10603075273949);
    }
}
