use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

type Grid = Vec<Vec<bool>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
    visited: HashSet<(usize, usize)>,
    loop_tracker: HashSet<(Direction, usize, usize)>,
    inside_loop: bool,
}

impl Guard {
    fn new(x: usize, y: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((y, x));
        let mut loop_tracker = HashSet::new();
        loop_tracker.insert((Direction::Up, y, x));
        Self {
            x,
            y,
            dir: Direction::Up,
            visited,
            loop_tracker,
            inside_loop: false,
        }
    }

    fn track_loop(&mut self) -> bool {
        if self.loop_tracker.contains(&(self.dir, self.y, self.x)) {
            self.inside_loop = true;
            false // report back that we have found a loop
        } else {
            self.loop_tracker.insert((self.dir, self.y, self.x));
            true
        }
    }

    fn step(&mut self, grid: &Grid, detect_loop: bool) -> bool {
        match self.dir {
            Direction::Up => {
                if self.y == 0 {
                    return false; // we will leave the grid
                } else if grid[self.y - 1][self.x] {
                    self.y -= 1;
                    if detect_loop {
                        if !self.track_loop() {
                            return false; // we are in a loop
                        }
                    } else {
                        self.visited.insert((self.y, self.x));
                    }
                } else {
                    self.dir = Direction::Right
                }
                true // let's go!
            }
            Direction::Down => {
                if self.y + 1 == grid.len() {
                    return false;
                } else if grid[self.y + 1][self.x] {
                    self.y += 1;
                    if detect_loop {
                        if !self.track_loop() {
                            return false;
                        }
                    } else {
                        self.visited.insert((self.y, self.x));
                    }
                } else {
                    self.dir = Direction::Left;
                }
                true
            }
            Direction::Left => {
                if self.x == 0 {
                    return false;
                } else if grid[self.y][self.x - 1] {
                    self.x -= 1;
                    if detect_loop {
                        if !self.track_loop() {
                            return false;
                        }
                    } else {
                        self.visited.insert((self.y, self.x));
                    }
                } else {
                    self.dir = Direction::Up
                }
                true
            }
            Direction::Right => {
                if self.x + 1 == grid.len() {
                    return false;
                } else if grid[self.y][self.x + 1] {
                    self.x += 1;
                    if detect_loop {
                        if !self.track_loop() {
                            return false;
                        }
                    } else {
                        self.visited.insert((self.y, self.x));
                    }
                } else {
                    self.dir = Direction::Down
                }
                true
            }
        }
    }
}

fn read_grid(input: &Path) -> (Guard, Vec<Vec<bool>>) {
    /* initial values */
    let mut guard_position = (0, 0); // top left with the first dimension being vertical
    let mut result = Vec::with_capacity(0);

    let reader = BufReader::new(File::open(input).unwrap());
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if y == 0 {
            result.reserve_exact(line.len()); // I know it is quadratic input
        }
        let values = line.chars().map(|c| c != '#').collect();
        if let Some(x) = line.find('^') {
            guard_position = (y, x);
        }
        result.push(values);
    }
    (Guard::new(guard_position.1, guard_position.0), result)
}

fn part1(input: &Path) -> usize {
    let (mut guard, grid) = read_grid(input);
    while guard.step(&grid, false) {}
    guard.visited.len()
}

fn part2(input: &Path) -> usize {
    let mut result = 0;
    let (mut guard, mut grid) = read_grid(input);
    let guard_copy = Guard::new(guard.x, guard.y);

    // record where the guard will be without obstacles
    while guard.step(&grid, false) {}

    // optimization: Only place obstacles where we know the guard will be
    for (y, x) in guard.visited {
        grid[y][x] = false; // place an obstacle
        let mut guard = guard_copy.clone(); // start from a clean slate
        while guard.step(&grid, true) {}
        if guard.inside_loop {
            result += 1;
        }
        grid[y][x] = true; // remove obstacle
    }

    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
