use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Glue,
}

#[derive(Debug, Clone)]
struct Solution {
    operators: Vec<Operator>,
}

impl Solution {
    fn new(num_operators: usize) -> Self {
        let operators = vec![Operator::Add; num_operators];
        Self { operators }
    }

    // go to next solution candidate. Return false when all options have been exhausted
    fn advance(&mut self, allow_glue: bool) -> bool {
        let mut overflows = 0;
        for i in 0..self.operators.len() {
            match self.operators[i] {
                Operator::Add => {
                    self.operators[i] = Operator::Mul;
                    break;
                }
                Operator::Mul => {
                    if allow_glue {
                        self.operators[i] = Operator::Glue;
                        break;
                    } else {
                        self.operators[i] = Operator::Add;
                        overflows += 1;
                    }
                }
                Operator::Glue => {
                    self.operators[i] = Operator::Add;
                    overflows += 1;
                }
            }
        }
        overflows != self.operators.len()
    }
}

impl Equation {
    fn is_solvable(&self, use_glue: bool) -> bool {
        let mut solution = Solution::new(self.operands.len() - 1);

        if self.test_solution(&solution) {
            return true;
        }

        while solution.advance(use_glue) {
            if self.test_solution(&solution) {
                return true;
            }
        }

        false
    }

    fn test_solution(&self, solution: &Solution) -> bool {
        let mut intermediate = self.operands[0];

        for (operand, operator) in self.operands[1..].iter().zip(&solution.operators) {
            match operator {
                Operator::Add => {
                    intermediate += operand;
                }
                Operator::Mul => {
                    intermediate *= operand;
                }
                Operator::Glue => {
                    intermediate = format!("{intermediate}{operand}").parse().unwrap();
                }
            }
        }

        intermediate == self.result
    }
}

fn get_equations(input: &Path) -> Vec<Equation> {
    let mut result = vec![];

    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let (eq_result, operands) = line.split_once(": ").unwrap();
        let eq_result = eq_result.parse().unwrap();
        let operands: Vec<usize> = operands.split(' ').map(|o| o.parse().unwrap()).collect();
        result.push(Equation {
            result: eq_result,
            operands,
        });
    }

    result
}

fn process_solvable_equations(input: &Path, use_glue: bool) -> usize {
    get_equations(input)
        .iter()
        .filter(|eq| eq.is_solvable(use_glue))
        .map(|eq| eq.result)
        .sum()
}

fn part1(input: &Path) -> usize {
    process_solvable_equations(input, false)
}

fn part2(input: &Path) -> usize {
    process_solvable_equations(input, true)
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
