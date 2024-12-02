use std::{
    cmp::{
        max, min,
        Ordering::{self},
    },
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

fn read_input(input: &Path) -> Vec<Vec<usize>> {
    let mut result = vec![];
    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let numbers: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        result.push(numbers);
    }
    result
}

fn determine_ordering(numbers: &[usize], use_dampener: bool) -> Ordering {
    if use_dampener {
        let left = min(numbers[0], numbers[1]);
        let right_values = numbers.last_chunk::<2>().unwrap();
        let right = max(right_values[0], right_values[1]);
        left.cmp(&right)
    } else {
        numbers[0].cmp(&numbers[1])
    }
}

fn is_safe(levels: &[usize], use_dampener: bool) -> bool {
    let ordering = determine_ordering(levels, use_dampener);
    if ordering == Ordering::Equal {
        return false; // neither ascending nor descending
    }
    if levels
        .iter()
        .zip(&levels[1..])
        .all(|(a, b)| a.cmp(b) == ordering && a.abs_diff(*b) <= 3)
    {
        true // all flawless!
    } else if use_dampener {
        let mut new_vec = Vec::with_capacity(levels.len() - 1);
        new_vec.extend_from_slice(&levels[1..]); // leave first element out
        assert_eq!(new_vec.len() + 1, levels.len());
        for (i, level) in levels.iter().enumerate() {
            // brute force: Try all report subsets
            if is_safe(&new_vec, false) {
                return true;
            }

            // swap next element out
            if i < new_vec.len() {
                new_vec[i] = *level;
            } // else: break
        }
        false // found no safe subset
    } else {
        false
    }
}

fn part1(input: &Path) -> usize {
    let records = read_input(input);
    let mut result = 0;
    for record in records {
        if is_safe(&record, false) {
            result += 1;
        }
    }
    result
}

fn part2(input: &Path) -> usize {
    let records = read_input(input);
    let mut result = 0;
    for record in &records {
        if is_safe(record, true) {
            result += 1;
        }
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
