use std::{
    cmp::Ordering,
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
    expected_solution: usize,
    current: usize,
    operands: &[usize],
    use_glue: bool,
) -> bool {
    match current.cmp(&expected_solution) {
        Ordering::Less => {
            // still room to grow
            if let Some((next, rest)) = operands.split_first() {
                solve_recursively(expected_solution, current + next, rest, use_glue)
                    || solve_recursively(expected_solution, current * next, rest, use_glue)
                    || use_glue
                        && solve_recursively(
                            expected_solution,
                            format!("{current}{next}").parse().unwrap(),
                            rest,
                            use_glue,
                        )
            } else {
                false // we didn't get high enough before we ran out of operands
            }
        }
        Ordering::Equal => operands.is_empty() || operands.iter().all(|op| *op <= 1), // allow for an arbitrary number of *1 +0 at the end
        Ordering::Greater => false, // we got too high
    }
}

fn part1(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| solve_recursively(eq.result, eq.operands[0], &eq.operands[1..], false))
        .map(|eq| eq.result)
        .sum()
}
fn part2(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| solve_recursively(eq.result, eq.operands[0], &eq.operands[1..], true))
        .map(|eq| eq.result)
        .sum()
}

pub fn main(input: &Path) -> (usize, usize) {
    let equations = read_equations(input);
    (part1(&equations), part2(&equations))
}
