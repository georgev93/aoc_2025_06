pub struct Homework<'a> {
    homework_sheet: &'a Vec<Vec<String>>,
    pub problems: usize,
    operands: usize,
}

impl<'a> Homework<'a> {
    pub fn new(homework_input: &'a Vec<Vec<String>>) -> Self {
        Self {
            homework_sheet: homework_input,
            problems: homework_input[0].len(),
            operands: homework_input.len() - 1,
        }
    }

    pub fn solve_problem(&self, prob_num: usize) -> u64 {
        let mut answer = self.homework_sheet[0][prob_num].parse::<u64>().unwrap();
        match self.homework_sheet[self.operands][prob_num].as_str() {
            "*" => {
                for operand in 1..self.operands {
                    answer *= self.homework_sheet[operand][prob_num]
                        .parse::<u64>()
                        .unwrap();
                }
            }
            "+" => {
                for operand in 1..self.operands {
                    answer += self.homework_sheet[operand][prob_num]
                        .parse::<u64>()
                        .unwrap();
                }
            }
            _ => {}
        }
        answer
    }
}
