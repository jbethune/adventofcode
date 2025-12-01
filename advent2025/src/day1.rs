use std::fs::File;
use std::io::{BufRead, BufReader};

use camino::Utf8Path as Path;

enum Direction {
    Left,
    Right,
}

struct Move {
    direction: Direction,
    amount: usize,
}

struct Safe {
    position: u8,           // must be lower than 100
    zero_count: usize,      // how many times we stop at 0
    zero_tick_count: usize, // how many times we cross 0
}

impl Safe {
    fn new() -> Self {
        Self {
            position: 50,
            zero_count: 0,
            zero_tick_count: 0,
        }
    }

    fn rotate(&mut self, mv: &Move) {
        let position: usize = self.position.into();
        let amount = mv.amount % 100;

        let new_position: usize = match mv.direction {
            Direction::Left => {
                if position <= amount && position != 0 {
                    self.zero_tick_count += 1;
                }
                (position + (100 - amount)) % 100
            }
            Direction::Right => {
                if position + amount > 99 {
                    self.zero_tick_count += 1;
                }
                (position + amount) % 100
            }
        };

        if new_position == 0 {
            self.zero_count += 1;
        }

        self.zero_tick_count += mv.amount / 100; // full rotations

        assert!(new_position < 100);
        self.position = new_position as u8;
    }
}

fn read_input(input: &Path) -> Vec<Move> {
    let reader = BufReader::new(File::open(input).unwrap());
    let mut result = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let (dir, amount) = line.split_at(1);
        let amount: usize = amount.parse().unwrap();
        let direction = if dir == "L" {
            Direction::Left
        } else {
            Direction::Right
        };
        result.push(Move { direction, amount });
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    let data = read_input(input);
    let mut safe = Safe::new();
    for mv in &data {
        safe.rotate(mv);
    }
    (safe.zero_count, safe.zero_tick_count)
}
