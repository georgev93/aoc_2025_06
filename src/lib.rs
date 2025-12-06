use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

mod file_parser;
use crate::file_parser::{FileParser, FileParserTrait};

mod homework;
mod homework_pt2;
use crate::homework::Homework;
use crate::homework_pt2::HomeworkPt2;

pub fn solve(input_file: &str) -> (u64, u64) {
    let input_grid = FileParser::new(input_file).parse_grid_strings();
    let my_homework = Homework::new(&input_grid);

    let mut total = 0u64;
    for i in 0..my_homework.problems {
        total += my_homework.solve_problem(i);
    }

    // let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    let result1 = Arc::new(AtomicU64::new(0));
    let result2 = Arc::new(AtomicU64::new(0));

    // let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(battery_banks.len());
    // for battery_bank in battery_banks {
    //     let result1_clone = Arc::clone(&result1);
    //     let result2_clone = Arc::clone(&result2);
    //     let handle = thread::spawn(move || {
    //         result1_clone.fetch_add(battery_bank.get_high_joltage(2), Ordering::SeqCst);
    //         result2_clone.fetch_add(battery_bank.get_high_joltage(12), Ordering::SeqCst);
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().expect("Thread panicked!");
    // }
    //
    // (
    //     result1.load(Ordering::Relaxed),
    //     result2.load(Ordering::Relaxed),
    // )
    (total, 0)
}

pub fn solve_pt1(input_file: &str) -> u64 {
    let input_grid = FileParser::new(input_file).parse_grid_strings();
    let my_homework = Arc::new(Homework::new(&input_grid));

    let total = Arc::new(AtomicU64::new(0));

    thread::scope(|s| {
        for i in 0..my_homework.problems {
            let total_clone = Arc::clone(&total);
            let my_homework_clone = Arc::clone(&my_homework);
            s.spawn(move || {
                total_clone.fetch_add(my_homework_clone.solve_problem(i), Ordering::SeqCst);
            });
        }
    });

    total.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (part_1, part_2) = solve("data/example.txt");
        assert_eq!(part_1, 4277556);
        assert_eq!(part_2, 0);
    }

    #[test]
    fn example_pts() {
        let (part_1, part_2) = solve("data/example.txt");
        assert_eq!(part_1, 4277556);
        assert_eq!(part_2, 0);
    }

    #[test]
    fn actual() {
        let (part_1, part_2) = solve("data/input.txt");
        assert_eq!(part_1, 6757749566978);
        assert_eq!(part_2, 0);
    }
}
