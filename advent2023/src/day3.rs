use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PotentialPartNumber {
    n: usize,
    pos: Position,
}

impl PotentialPartNumber {
    fn new(n: usize, x: usize, y: usize) -> Self {
        let pos = Position { x, y };
        Self { n, pos }
    }

    fn is_activated_by(&self, symbol_pos: &Position) -> bool {
        if self.pos.y.abs_diff(symbol_pos.y) > 1 {
            return false;
        }
        let left_x = self.pos.x.saturating_sub(1);
        let right_x = self.pos.x.saturating_add(self.n.to_string().len());
        left_x <= symbol_pos.x && symbol_pos.x <= right_x
    }
}

fn make_number(digits: &[u8]) -> usize {
    let mut result = 0;
    for digit in digits {
        result *= 10;
        result += *digit as usize;
    }
    result
}

fn read_numbers(input: &Path) -> Vec<PotentialPartNumber> {
    let mut result = vec![];
    let reader = BufReader::new(File::open(input).unwrap());
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut start: Option<usize> = None;
        let mut digits: Vec<u8> = vec![];
        for (x, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                if start.is_none() {
                    start = Some(x);
                }
                digits.push(digit as u8);
            } else if let Some(x) = start {
                let number = PotentialPartNumber::new(make_number(&digits), x, y);
                digits.clear();
                result.push(number);
                start = None;
            }
        }
        if let Some(x) = start {
            let number = PotentialPartNumber::new(make_number(&digits), x, y);
            result.push(number);
        }
    }
    result
}

fn gather_positions<F>(input: &Path, condition: F) -> Vec<Position>
where
    F: Fn(char) -> bool,
{
    let mut result = vec![];
    let reader = BufReader::new(File::open(input).unwrap());
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if condition(c) {
                result.push(Position { x, y });
            }
        }
    }
    result
}

fn gather_activating_positions(input: &Path) -> Vec<Position> {
    gather_positions(input, |c| !(c.is_ascii_digit() || c == '.'))
    /*
    let mut result = vec![];
    let reader = BufReader::new(File::open(input).unwrap());
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if !(c.is_ascii_digit() || c == '.') {
                result.push(Position { x, y });
            }
        }
    }
    result
    */
}

fn gather_gears(input: &Path) -> Vec<Position> {
    gather_positions(input, |c| c == '*')
}

fn part1(input: &Path) -> usize {
    let potential_part_numbers = read_numbers(input);
    let activators = gather_activating_positions(input);

    // let tester = PotentialPartNumber::new(421, 2, 2);
    // assert!(tester.is_activated_by(&Position { x: 6, y: 1 }));

    let active_filter = |part_number: &PotentialPartNumber| {
        if activators
            .iter()
            .any(|activator| part_number.is_activated_by(activator))
        {
            Some(part_number.n)
        } else {
            None
        }
    };

    // dbg!(potential_part_numbers.iter().map(|no| no.n).sum::<usize>());
    potential_part_numbers
        .iter()
        .filter_map(active_filter)
        .sum()
}

fn part2(input: &Path) -> usize {
    let mut result = 0;
    let potential_part_numbers = read_numbers(input); // a bit wasteful to not reuse result
    let gears = gather_gears(input);
    for gear in gears {
        for (i, n1) in potential_part_numbers.iter().enumerate() {
            if !n1.is_activated_by(&gear) {
                continue;
            }
            for n2 in potential_part_numbers.iter().skip(i + 1) {
                if n2.is_activated_by(&gear) {
                    let gear_ratio = n1.n * n2.n;
                    result += gear_ratio;
                }
            }
        }
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
