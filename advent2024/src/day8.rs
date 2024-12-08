use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

type Frequency = char;

// (0,0) is top-left and given (y,x) we have row y and column x
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Diff {
    w: isize,
    h: isize,
}

impl Diff {
    fn flip(&self) -> Self {
        Self {
            w: -self.w,
            h: -self.h,
        }
    }
}

impl Position {
    fn diff(&self, rhs: &Self) -> Diff {
        Diff {
            w: self.x as isize - rhs.x as isize,
            h: self.y as isize - rhs.y as isize,
        }
    }

    fn shift(&self, diff: &Diff, grid_size: usize) -> Option<Position> {
        let x: usize = (self.x as isize).checked_add(diff.w)?.try_into().ok()?;
        let y: usize = (self.y as isize).checked_add(diff.h)?.try_into().ok()?;
        if x < grid_size && y < grid_size {
            Some(Position { x, y })
        } else {
            None
        }
    }
}

struct RadioMap {
    grid_size: usize,
    stations: HashMap<Frequency, Vec<Position>>,
}

fn read_radio_positions(input: &Path) -> RadioMap {
    let mut stations = HashMap::new();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut grid_size = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if y == 0 {
            grid_size = line.len();
        }
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                stations
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(Position { x, y });
            }
        }
    }
    RadioMap {
        grid_size,
        stations,
    }
}

fn part1(map: &RadioMap) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();
    let n = map.grid_size;
    for positions in map.stations.values() {
        for i in 0..positions.len() {
            let a = &positions[i];
            for b in &positions[i + 1..] {
                let diff = a.diff(b);

                if let Some(pos) = a.shift(&diff, n) {
                    antinodes.insert(pos);
                }
                let diff = diff.flip();
                if let Some(pos) = b.shift(&diff, n) {
                    antinodes.insert(pos);
                }
            }
        }
    }
    antinodes.len()
}

fn part2(map: &RadioMap) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();
    let n = map.grid_size;
    for positions in map.stations.values() {
        for i in 0..positions.len() {
            for b in &positions[i + 1..] {
                let mut a = positions[i].clone();
                antinodes.insert(a.clone());

                let diff = a.diff(b);
                while let Some(new_a) = a.shift(&diff, n) {
                    antinodes.insert(new_a.clone());
                    a = new_a;
                }

                let diff = diff.flip();
                let mut a = positions[i].clone();
                while let Some(new_a) = a.shift(&diff, n) {
                    antinodes.insert(new_a.clone());
                    a = new_a;
                }
            }
        }
    }
    antinodes.len()
}

pub fn main(input: &Path) -> (usize, usize) {
    let radios = read_radio_positions(input);
    (part1(&radios), part2(&radios))
}
