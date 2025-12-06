#[derive(Debug)]
struct ProblemCoords {
    start: usize,
    width: usize,
    operation: char,
}

pub struct Homework<'a> {
    homework_sheet: &'a Vec<Vec<char>>,
    pub num_of_problems: usize,
    problems: Vec<ProblemCoords>,
    operands_height: usize,
}

impl<'a> Homework<'a> {
    pub fn new(homework_input: &'a Vec<Vec<char>>) -> Self {
        let rows = homework_input.len();
        let cols = homework_input[0].len();
        let mut last_problem_coords = ProblemCoords {
            start: 0,
            width: 0,
            operation: homework_input[rows - 1][0],
        };
        let mut vec_of_problem_coords = Vec::<ProblemCoords>::new();

        for i in 1..cols {
            if homework_input[rows - 1][i] != ' ' {
                last_problem_coords.width = i - 1 - last_problem_coords.start;
                vec_of_problem_coords.push(last_problem_coords);
                last_problem_coords = ProblemCoords {
                    start: i,
                    width: 0,
                    operation: homework_input[rows - 1][i],
                }
            }
        }
        last_problem_coords.width = cols - last_problem_coords.start;
        vec_of_problem_coords.push(last_problem_coords);

        Self {
            homework_sheet: homework_input,
            num_of_problems: vec_of_problem_coords.len(),
            problems: vec_of_problem_coords,
            operands_height: homework_input.len() - 1,
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
                for i in 1..operands.len() {
                    answer *= operands[i];
                }
            }
            '+' => {
                for i in 1..operands.len() {
                    answer += operands[i];
                }
            }
            _ => {}
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
                for i in 1..operands.len() {
                    answer *= operands[i];
                }
            }
            '+' => {
                for i in 1..operands.len() {
                    answer += operands[i];
                }
            }
            _ => {}
        }
        answer
    }
}
