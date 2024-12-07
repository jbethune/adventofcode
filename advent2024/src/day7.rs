use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

fn read_equations(input: &Path) -> Vec<Equation> {
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

// depth-first search that reuses intermediate results
fn solve_recursively(
    equation: &Equation,
    current: usize,
    operands: &[usize],
    use_glue: bool,
) -> bool {
    if let Some((next, rest)) = operands.split_first() {
        solve_recursively(equation, current + next, rest, use_glue)
            || solve_recursively(equation, current * next, rest, use_glue)
            || use_glue
                && solve_recursively(
                    equation,
                    format!("{current}{next}").parse().unwrap(),
                    rest,
                    use_glue,
                )
    } else {
        current == equation.result
    }
}

fn part1(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| solve_recursively(eq, eq.operands[0], &eq.operands[1..], false))
        .map(|eq| eq.result)
        .sum()
}
fn part2(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| solve_recursively(eq, eq.operands[0], &eq.operands[1..], true))
        .map(|eq| eq.result)
        .sum()
}

pub fn main(input: &Path) -> (usize, usize) {
    let equations = read_equations(input);
    (part1(&equations), part2(&equations))
}
