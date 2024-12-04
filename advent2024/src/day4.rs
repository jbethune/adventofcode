use std::{fs::read_to_string, str::Chars};

use camino::Utf8Path as Path;

fn count(s: &str, pattern: &'static str) -> usize {
    s.matches(pattern).count()
}

fn transform_vertical(old_lines: &[&str]) -> Vec<String> {
    let n = old_lines[0].len(); // assuming equally sized strings
    let mut iters: Vec<Chars> = old_lines.iter().map(|s| s.chars()).collect();
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        let mut new_line = String::with_capacity(n);
        for iter in &mut iters {
            let c: char = iter.next().unwrap();
            new_line.push(c);
        }
        result.push(new_line);
    }
    result
}

// shift and padd the original strings so that it's easy to process them afterwards
fn transform_diagonal(old_lines: &[&str], ascending_diagonal: bool) -> Vec<String> {
    let n = old_lines.len();
    let mut shifted: Vec<String> = Vec::with_capacity(n);
    if ascending_diagonal {
        for (i, s) in old_lines.iter().enumerate() {
            let mut new_string = " ".repeat(n - i);
            new_string.push_str(s);
            let right_padding = " ".repeat(i);
            new_string.push_str(&right_padding);
            shifted.push(new_string);
        }
    } else {
        //then: descending diagonal
        for (i, s) in old_lines.iter().enumerate() {
            let mut new_string = " ".repeat(i);
            new_string.push_str(s);
            let right_padding = " ".repeat(n - i);
            new_string.push_str(&right_padding);
            shifted.push(new_string);
        }
    }

    let slices: Vec<&str> = shifted.iter().map(|s| s.as_str()).collect();
    transform_vertical(&slices)
}

fn part1(input: &Path) -> usize {
    let text = read_to_string(input).unwrap();
    let mut result = 0;
    let lines: Vec<&str> = text.lines().collect();

    // lines is horizontal
    for line in &lines {
        result += count(line, "XMAS");
        result += count(line, "SAMX");
    }
    let vertical = transform_vertical(&lines);
    for line in vertical {
        result += count(&line, "XMAS");
        result += count(&line, "SAMX");
    }
    let diagonal = transform_diagonal(&lines, true);
    for line in diagonal {
        result += count(&line, "XMAS");
        result += count(&line, "SAMX");
    }

    let diagonal = transform_diagonal(&lines, false);
    for line in diagonal {
        result += count(&line, "XMAS");
        result += count(&line, "SAMX");
    }
    result
}

fn part2(input: &Path) -> usize {
    let text = read_to_string(input).unwrap();
    let n = text.find("\n").unwrap(); // assuming square shape

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(n);
    for line in text.lines() {
        let v = line.chars().collect();
        grid.push(v);
    }

    let mut result = 0;
    for (row, line) in grid.iter().enumerate().skip(1) {
        if row + 1 == n {
            continue; // grid boundary
        }
        for (col, c) in line.iter().enumerate().skip(1) {
            if col + 1 == n {
                continue; // grid boundary
            }
            if *c == 'A' {
                // top left, top right, bottom left, bottom right
                let tl = grid[row + 1][col - 1];
                let tr = grid[row + 1][col + 1];
                let bl = grid[row - 1][col - 1];
                let br = grid[row - 1][col + 1];
                let has_descending = tl == 'M' && br == 'S' || tl == 'S' && br == 'M';
                let has_ascending = bl == 'M' && tr == 'S' || bl == 'S' && tr == 'M';
                if has_ascending && has_descending {
                    result += 1;
                }
            }
        }
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
