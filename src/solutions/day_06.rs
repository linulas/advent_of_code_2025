use crate::day::Solution;

#[derive(Debug)]
pub enum ArithmeticOperation {
    Add,
    Multiply,
}

impl From<char> for ArithmeticOperation {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Problem {
    pub column: Vec<String>,
    pub operation: ArithmeticOperation,
}

impl Problem {
    pub fn solve(&self) -> u64 {
        match self.operation {
            ArithmeticOperation::Add => self.numbers().iter().sum::<u64>(),
            ArithmeticOperation::Multiply => self.numbers().iter().product::<u64>(),
        }
    }
    pub fn numbers(&self) -> Vec<u64> {
        self.column
            .iter()
            .map(|s| s.trim().parse::<u64>().expect("should be a number"))
            .collect::<Vec<u64>>()
    }
    pub fn column_width(&self) -> usize {
        self.column.iter().fold(0, |acc, n| acc.max(n.len()))
    }
    pub fn convert_to_cephalopod(&mut self) {
        let mut column: Vec<String> = vec![];

        (0..self.column_width()).rev().for_each(|i| {
            let mut number_string = String::new();

            (0..self.column.len()).for_each(|j| {
                number_string.push(
                    self.column[j]
                        .to_string()
                        .chars()
                        .nth(i)
                        .expect("should be a char"),
                );
            });

            column.push(number_string);
        });

        self.column = column;
    }
}

pub struct MathHomework {
    pub problems: Vec<Problem>,
}

impl MathHomework {
    pub fn new(input: &'static str) -> Self {
        let mut lines = input.lines().collect::<Vec<&str>>();
        let last_line = lines.pop().expect("input should have at least one line");
        let mut operation_indecies: Vec<usize> = vec![];
        let mut columns: Vec<Vec<String>> = vec![];

        last_line.chars().enumerate().for_each(|(i, c)| match c {
            '+' => operation_indecies.push(i),
            '*' => operation_indecies.push(i),
            _ => (),
        });

        operation_indecies
            .iter()
            .enumerate()
            .for_each(|(i, operation_index)| {
                let mut column: Vec<String> = vec![];

                lines.iter().for_each(|line| {
                    if i == operation_indecies.len() - 1 {
                        column.push(line[*operation_index..].to_string());
                    } else {
                        column.push(
                            line[*operation_index..operation_indecies[i + 1] - 1].to_string(),
                        );
                    }
                });

                columns.push(column);
            });

        Self {
            problems: operation_indecies
                .iter()
                .enumerate()
                .map(|(i, operation_idex)| Problem {
                    column: columns[i].clone(),
                    operation: ArithmeticOperation::from(
                        last_line
                            .chars()
                            .nth(*operation_idex)
                            .expect("should be a char"),
                    ),
                })
                .collect(),
        }
    }
}

impl Solution<u64, u64> for MathHomework {
    fn part_one(&mut self) -> u64 {
        self.problems.iter().map(Problem::solve).sum()
    }
    fn part_two(&mut self) -> u64 {
        self.problems
            .iter_mut()
            .map(|p| {
                p.convert_to_cephalopod();
                p.solve()
            })
            .sum()
    }
}

