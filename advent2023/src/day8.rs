use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

type NodeName = (char, char, char);
type Branches = HashMap<NodeName, (NodeName, NodeName)>;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

struct Map {
    steps: Vec<Direction>,
    branches: Branches,
}

fn parse_line(line: &str) -> (NodeName, NodeName, NodeName) {
    let (from, to) = line.split_once(" = ").unwrap();
    let from: Vec<char> = from.chars().collect();
    let from = (from[0], from[1], from[2]);

    let (left, right) = to.split_once(", ").unwrap();
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let left = (left[1], left[2], left[3]);
    let right = (right[0], right[1], right[2]);

    (from, left, right)
}

fn read_map_file(input: &Path) -> Map {
    let mut steps: Option<Vec<Direction>> = None;
    let mut branches = HashMap::new();
    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        if steps.is_some() {
            if !line.is_empty() {
                let (from, left, right) = parse_line(&line);
                branches.insert(from, (left, right));
            }
        } else {
            let mut values = vec![];
            for char in line.trim().chars() {
                values.push(match char {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("invalid stp"),
                });
            }
            steps.replace(values);
        }
    }
    Map {
        steps: steps.unwrap(),
        branches,
    }
}

fn find_start_nodes(map: &Map) -> Vec<NodeName> {
    let mut result = vec![];
    for node in map.branches.keys() {
        if node.2 == 'A' {
            result.push(*node);
        }
    }
    result
}

fn part1(input: &Path) -> usize {
    let map = read_map_file(input);
    let mut current: NodeName = ('A', 'A', 'A');
    let destination: NodeName = ('Z', 'Z', 'Z');
    let mut steps = map.steps.iter().cycle();
    let mut count = 0;
    while current != destination {
        let direction = steps.next().unwrap();
        let options = map.branches[&current];
        current = match direction {
            Direction::Left => options.0,
            Direction::Right => options.1,
        };
        count += 1;
    }
    count
}

fn part2(input: &Path) -> usize {
    let mut count = 0;
    let map = read_map_file(input);
    let mut current_nodes: Vec<NodeName> = find_start_nodes(&map);
    let mut steps = map.steps.iter().cycle();
    // dbg!(&map.steps);
    while !current_nodes.iter().all(|n| n.2 == 'Z') {
        // dbg!(&current_nodes);
        let direction = steps.next().unwrap();

        current_nodes = {
            let mut new_nodes = vec![];
            match direction {
                Direction::Left => {
                    for node in current_nodes {
                        new_nodes.push(map.branches[&node].0);
                    }
                }
                Direction::Right => {
                    for node in current_nodes {
                        new_nodes.push(map.branches[&node].1);
                    }
                }
            }
            new_nodes
        };
        count += 1;
    }
    count
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
