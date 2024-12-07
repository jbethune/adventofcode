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

fn unglue(current: usize, last: usize, operands: &[usize]) -> bool {
    let current_str = current.to_string();
    let last_str = last.to_string();
    if let Some(new_str) = current_str.strip_suffix(&last_str) {
        solve_recursively(new_str.parse().unwrap(), operands, true)
    } else {
        false // can't be unglued
    }
}

// work backwards from the destination to the first operand to drastically reduce the search space
fn solve_recursively(current: usize, remaining_operands: &[usize], use_glue: bool) -> bool {
    if let Some((last, rest)) = remaining_operands.split_last() {
        match last.cmp(&current) {
            Ordering::Less => {
                let last = *last;

                // determine which operations could explain the (intermediate) result and follow the paths:
                current.rem_euclid(last) == 0 && solve_recursively(current / last, rest, use_glue)
                    || solve_recursively(current - last, rest, use_glue) // we already checked last < current
                    || use_glue && unglue(current, last, rest)
            }
            Ordering::Equal => {
                rest.is_empty() || rest.iter().all(|op| *op == 1) // we can divide by 1 any number of times
            }
            Ordering::Greater => false, // overshot the target
        }
    } else {
        false // didn't converge on first operand
    }
}

fn part1(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| solve_recursively(eq.result, &eq.operands, false))
        .map(|eq| eq.result)
        .sum()
}
fn part2(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| solve_recursively(eq.result, &eq.operands, true))
        .map(|eq| eq.result)
        .sum()
}

pub fn main(input: &Path) -> (usize, usize) {
    let equations = read_equations(input);
    (part1(&equations), part2(&equations))
}
