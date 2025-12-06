#[derive(Debug)]
struct ProblemCoords {
    start: usize,
    width: usize,
    operation: char,
}

pub struct Homework {
    homework_sheet: Vec<Vec<char>>,
    pub num_of_problems: usize,
    problems: Vec<ProblemCoords>,
    operands_height: usize,
}

impl Homework {
    pub fn new(homework_input: &str) -> Self {
        let mut homework_vec: Vec<Vec<char>> = Vec::new();
        for line in homework_input.lines() {
            let line_of_chars = line
                .as_bytes()
                .iter()
                .map(|b| *b as char)
                .collect::<Vec<char>>();
            homework_vec.push(line_of_chars);
        }
        let rows = homework_vec.len();
        let cols = homework_vec[0].len();
        let mut last_problem_coords = ProblemCoords {
            start: 0,
            width: 0,
            operation: homework_vec[rows - 1][0],
        };
        let mut vec_of_problem_coords = Vec::<ProblemCoords>::with_capacity(1500);

        for i in 1..cols {
            if homework_vec[rows - 1][i] != ' ' {
                last_problem_coords.width = i - 1 - last_problem_coords.start;
                vec_of_problem_coords.push(last_problem_coords);
                last_problem_coords = ProblemCoords {
                    start: i,
                    width: 0,
                    operation: homework_vec[rows - 1][i],
                }
            }
        }
        last_problem_coords.width = cols - last_problem_coords.start;
        vec_of_problem_coords.push(last_problem_coords);

        Self {
            operands_height: &homework_vec.len() - 1,
            homework_sheet: homework_vec,
            num_of_problems: vec_of_problem_coords.len(),
            problems: vec_of_problem_coords,
        }
    }

    pub fn solve_problem_ceph(&self, prob_num: usize) -> u64 {
        let this_problem = &self.problems[prob_num];
        let mut operands = Vec::<u64>::with_capacity(this_problem.width);

        for col in 0..this_problem.width {
            let mut number = String::with_capacity(self.operands_height);
            for row in 0..self.operands_height {
                number.push(self.homework_sheet[row][col + this_problem.start]);
            }
            operands.push(number.trim_ascii().parse::<u64>().unwrap());
        }

        let mut answer = operands[0];
        match this_problem.operation {
            '*' => {
                for operand in operands.iter().skip(1) {
                    answer *= operand;
                }
            }
            '+' => {
                for operand in operands.iter().skip(1) {
                    answer += operand;
                }
            }
            _ => unreachable!(),
        }
        answer
    }

    pub fn solve_problem_human(&self, prob_num: usize) -> u64 {
        let this_problem = &self.problems[prob_num];
        let mut operands = Vec::<u64>::with_capacity(self.operands_height);

        for row in 0..self.operands_height {
            let mut number = String::with_capacity(this_problem.width);
            for col in 0..this_problem.width {
                number.push(self.homework_sheet[row][col + this_problem.start]);
            }
            operands.push(number.trim_ascii().parse::<u64>().unwrap());
        }

        let mut answer = operands[0];
        match this_problem.operation {
            '*' => {
                for operand in operands.iter().skip(1) {
                    answer *= operand;
                }
            }
            '+' => {
                for operand in operands.iter().skip(1) {
                    answer += operand;
                }
            }
            _ => unreachable!(),
        }
        answer
    }
}
