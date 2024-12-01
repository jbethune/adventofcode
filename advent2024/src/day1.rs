use std::fs::File;
use std::io::{BufRead, BufReader};

use camino::Utf8Path as Path;

fn get_numbers(input: &Path) -> (Vec<usize>, Vec<usize>) {
    let mut a = vec![];
    let mut b = vec![];
    let fd = File::open(input).unwrap();
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let line = line.unwrap();
        let (x, y) = line.split_once("   ").unwrap();
        a.push(x.parse().unwrap());
        b.push(y.parse().unwrap());
    }
    (a, b)
}

fn part1(input: &Path) -> usize {
    let (mut a, mut b) = get_numbers(input);
    a.sort();
    b.sort();
    let mut result = 0;
    for (x, y) in a.iter().zip(b) {
        result += x.abs_diff(y)
    }
    result
}

fn part2(input: &Path) -> usize {
    let (a, b) = get_numbers(input);
    let mut result = 0;
    for x in a {
        result += x * b.iter().fold(0, |acc, z| acc + if *z == x { 1 } else { 0 })
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
