use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Extend;

use crate::direction::Direction;
use crate::matrix::Matrix;

pub fn matrix_from_file(path: &str) -> Matrix<u8> {
    let mut values: Vec<u8> = vec![];

    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    let mut n_cols = 0;
    let mut n_rows = 0;

    for line in reader.lines() {
        n_rows += 1;
        let line = line.unwrap();
        n_cols = line.trim().len();
        values.extend(
            line.chars()
                .map(TryInto::try_into)
                .map(Result::<u8, _>::unwrap),
        );
    }
    Matrix::<u8> {
        n_cols,
        n_rows,
        values,
    }
}

pub(crate) fn visibility_map(heights: &Matrix<u8>) -> Matrix<bool> {
    let n_rows = heights.n_rows;
    let n_cols = heights.n_cols;
    let mut result = Matrix::new(n_rows, n_cols, false);
    result.assign_col(0, true);
    result.assign_col(n_cols - 1, true);
    result.assign_row(0, true);
    result.assign_row(n_rows - 1, true);

    fn indices(rev: bool, max: usize) -> Vec<usize> {
        if rev {
            (0..max).rev().collect()
        } else {
            (0..max).collect()
        }
    }

    let mut update_light = |xrev: bool, yrev: bool, rowwise: bool| {
        for y in indices(yrev, n_rows) {
            let mut side_maximum = 0;
            for x in indices(xrev, n_cols) {
                let value = heights.get(x, y, rowwise);
                if value > side_maximum {
                    side_maximum = value;
                    result.set(true, x, y, rowwise);
                }
            }
        }
    };
    update_light(false, false, true); // light from east
    update_light(false, true, false);
    update_light(true, false, true); // light from west
    update_light(true, true, false);

    result
}

pub(crate) fn best_view_score(heights: &Matrix<u8>) -> usize {
    let n_rows = heights.n_rows;
    let n_cols = heights.n_cols;

    let mut highest = 0;
    for y in 0..n_rows {
        for x in 0..n_cols {
            let value = heights.get(x, y, true);
            let mut score = 0;
            for dir in &[
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South,
            ] {
                let mut tally = 0;
                for neighbor in heights.neighbours(x, y, *dir) {
                    if neighbor >= value {
                        tally += 1; // we can see the high tree, but not further
                        break;
                    }
                    tally += 1
                }
                if score == 0 {
                    score = tally;
                } else {
                    score *= tally;
                }
            }
            if score > highest {
                highest = score;
            }
        }
    }
    highest
}
