use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::direction::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn project(&self, direction: Direction) -> Self {
        let mut result = self.clone();
        match direction {
            Direction::North => result.y += 1,
            Direction::South => result.y -= 1,
            Direction::East => result.x += 1,
            Direction::West => result.x -= 1,
        }
        result
    }

    pub fn too_far(&self, other: &Coordinate) -> bool {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);
        x > 1 || y > 1
    }

    pub fn follow(&mut self, predecessor: &Coordinate) {
        let pred = predecessor;
        if self.too_far(pred) {
            self.x += if pred.x > self.x {
                1
            } else if pred.x < self.x {
                -1
            } else {
                0
            };
            self.y += if pred.y > self.y {
                1
            } else if pred.y < self.y {
                -1
            } else {
                0
            };
        }
    }
}

pub(crate) fn read_moves_from_file(path: &str) -> Vec<Direction> {
    let mut result = vec![];
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let direction = match parts[0] {
            "D" => Direction::South,
            "U" => Direction::North,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => panic!("Unknown direction: {}", parts[0]),
        };
        let times = parts[1].parse::<usize>().unwrap();
        for _ in 0..times {
            result.push(direction);
        }
    }
    result
}

pub(crate) fn visited_places(rope_length: usize, moves: &Vec<Direction>) -> HashSet<Coordinate> {
    let mut knots = vec![Coordinate { x: 0, y: 0 }; rope_length];
    let mut result = HashSet::new();
    result.insert(knots.last().cloned().unwrap());
    for dir in moves {
        for i in 0..rope_length {
            if i == 0 {
                knots[0] = knots[0].project(*dir);
            } else {
                let pred = knots[i - 1];
                knots[i].follow(&pred);
            }
        }
        result.insert(knots.last().cloned().unwrap());
    }
    result
}
