use std::fs::File;
use std::io::{BufRead, BufReader};

use camino::Utf8Path as Path;

const NUMBERS: [&str; 10] = [
    "ZERO", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part1(input: &Path) -> usize {
    let mut result = 0;

    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let mut left: Option<usize> = None;
        let mut right = 0;
        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                right = n as usize;
                if left.is_none() {
                    left.replace(n as usize);
                }
            }
        }
        let sum = left.unwrap_or(0) * 10 + right;
        result += sum;
    }
    result
}

fn part2(input: &Path) -> usize {
    let mut result = 0;

    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let mut left: Option<usize> = None;
        let mut right = 0;
        let mut slice = line.as_str();
        while let Some(c) = slice.chars().next() {
            if let Some(n) = c.to_digit(10) {
                right = n as usize;
                if left.is_none() {
                    left.replace(n as usize);
                }
            } else {
                for (i, number) in NUMBERS.iter().enumerate() {
                    if slice.starts_with(number) {
                        right = i;
                        if left.is_none() {
                            left.replace(i);
                        }
                    }
                }
            }
            slice = &slice[1..];
        }
        let sum = left.unwrap_or(0) * 10 + right;
        result += sum;
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
