use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

use crate::direction::Direction;
use crate::matrix::Matrix;
use crate::rope::Coordinate;

pub struct Map {
    start: Coordinate,
    end: Coordinate,
    heights: Matrix<u8>,
}

pub(crate) fn read_heightmap_from_file(path: &str) -> Map {
    let data = read_to_string(path).unwrap();
    let lines: Vec<&str> = data.split('\n').collect();
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let mut matrix = Matrix::new(n_rows, n_cols, 0);
    matrix.values.clear();
    let mut start = Coordinate { x: 0, y: 0 };
    let mut end = Coordinate { x: 0, y: 0 };
    for c in data.bytes() {
        match c {
            b'S' => {
                let x = (matrix.values.len() % n_cols) as isize;
                let y = (matrix.values.len() % n_rows) as isize;
                start = Coordinate { x, y };
                matrix.values.push(0);
            }
            b'E' => {
                let x = (matrix.values.len() % n_cols) as isize;
                let y = (matrix.values.len() % n_rows) as isize;
                end = Coordinate { x, y };
                matrix.values.push(25);
            }
            b'\n' => {}
            _ => {
                matrix.values.push(c - 97);
            }
        }
    }

    Map {
        start,
        end,
        heights: matrix,
    }
}

pub(crate) fn shortest_path_length(map: &Map) -> usize {
    let mut visited = HashSet::<Coordinate>::new();
    let mut queue: VecDeque<Coordinate> = vec![];
    let mut steps = 0;
    queue.push_back(map.start);
    while queue.len() > 0 {
        let mut new_queue = vec![];
        for 
    let Some(coordinate) = queue.pop_front() {
        visited.insert(coordinate);


        for dir in &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            // TODO Baustelle
        }
    }
    0
}
