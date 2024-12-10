use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

#[derive(Debug, Clone, Default)]
struct BigBitSet {
    bits: [usize; 3], // should be dynamic size for real-world use
}

impl BigBitSet {
    fn set(&mut self, index: usize) {
        const DIV: usize = usize::BITS as usize;
        self.bits[index / DIV] |= 1 << (index % DIV);
    }

    fn union(&mut self, other: &BigBitSet) {
        for i in 0..self.bits.len() {
            self.bits[i] |= other.bits[i];
        }
    }
    fn len(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }
}

type HeightMap = Vec<Vec<u8>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn read_map(input: &Path) -> (HeightMap, Vec<Position>, HashMap<Position, usize>) {
    let mut map = vec![];
    let mut heads = Vec::new();
    let mut peaks = HashMap::new();

    let reader = BufReader::new(File::open(input).unwrap());
    for (y, line) in reader.lines().enumerate() {
        let mut heights = vec![];
        for (x, c) in line.unwrap().chars().enumerate() {
            let height: u8 = c.to_digit(10).unwrap().try_into().unwrap();
            heights.push(height);
            if height == 0 {
                heads.push(Position::new(x, y));
            } else if height == 9 {
                peaks.insert(Position::new(x, y), peaks.len());
            }
        }
        map.push(heights);
    }
    (map, heads, peaks)
}

fn go_downhill(current: Position, map: &HeightMap, grid: &mut Vec<Vec<BigBitSet>>) {
    let height = map[current.y][current.x];
    let current_config = grid[current.y][current.x].clone();

    // move north
    if let Some(y) = current.y.checked_sub(1) {
        if map[y][current.x] + 1 == height {
            grid[y][current.x].union(&current_config);
            go_downhill(Position { x: current.x, y }, map, grid);
        }
    }

    // move south
    let y = current.y + 1;
    if let Some(row) = map.get(y) {
        if row[current.x] + 1 == height {
            grid[y][current.x].union(&current_config);
            go_downhill(Position { x: current.x, y }, map, grid);
        }
    }

    // move west
    if let Some(x) = current.x.checked_sub(1) {
        if map[current.y][x] + 1 == height {
            grid[current.y][x].union(&current_config);
            go_downhill(Position { x, y: current.y }, map, grid);
        }
    }

    // move east
    let x = current.x + 1;
    if let Some(field) = map[current.y].get(x) {
        if field + 1 == height {
            grid[current.y][x].union(&current_config);
            go_downhill(Position { x, y: current.y }, map, grid);
        }
    }
}

fn go_uphill(current: Position, map: &HeightMap) -> usize {
    let height = map[current.y][current.x];
    if height == 9 {
        return 1; // end of recursion
    }

    let mut paths_up = 0;

    // move north
    if let Some(y) = current.y.checked_sub(1) {
        if map[y][current.x] == 1 + height {
            paths_up += go_uphill(Position { x: current.x, y }, map);
        }
    }
    // move south
    let y = current.y + 1;
    if let Some(row) = map.get(y) {
        if row[current.x] == 1 + height {
            paths_up += go_uphill(Position { x: current.x, y }, map);
        }
    }

    // move west
    if let Some(x) = current.x.checked_sub(1) {
        if map[current.y][x] == 1 + height {
            paths_up += go_uphill(Position { x, y: current.y }, map);
        }
    }

    // move east
    let x = current.x + 1;
    if let Some(field) = map[current.y].get(x) {
        if *field == 1 + height {
            paths_up += go_uphill(Position { x, y: current.y }, map);
        }
    }
    paths_up
}

fn part1(map: &HeightMap, heads: &[Position], peaks: &HashMap<Position, usize>) -> usize {
    let mut grid: Vec<Vec<BigBitSet>> = Vec::with_capacity(map.len());
    for _ in map {
        grid.push(vec![BigBitSet::default(); map[0].len()]);
    }

    for (peak, idx) in peaks {
        grid[peak.y][peak.x].set(*idx);
        go_downhill(
            Position {
                x: peak.x,
                y: peak.y,
            },
            map,
            &mut grid,
        );
    }
    heads.iter().map(|head| grid[head.y][head.x].len()).sum()
}

fn part2(map: &HeightMap, heads: &[Position]) -> usize {
    heads.iter().map(|head| go_uphill(*head, map)).sum()
}

pub fn main(input: &Path) -> (usize, usize) {
    let (map, heads, peaks) = read_map(input);
    (part1(&map, &heads, &peaks), part2(&map, &heads))
}
