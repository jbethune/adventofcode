use std::cmp::{max, min};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

use camino::Utf8Path as Path;

const LOOKUPS: &[&str] = &[
    // "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

#[derive(Debug, Default)]
struct Mapping {
    source: Range<usize>,
    destination: Range<usize>,
}

#[derive(Debug, Default)]
struct Maps {
    seeds: Vec<usize>,
    mappings: HashMap<(String, String), Vec<Mapping>>,
}

fn read_maps(input: &Path) -> Maps {
    let reader = BufReader::new(File::open(input).unwrap());
    let mut lines = reader.lines();
    let seeds_line = lines.next().unwrap().unwrap();
    let (_, seed_values) = seeds_line.split_once(": ").unwrap();
    let seeds: Vec<usize> = seed_values
        .trim()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut result = Maps::default();
    result.seeds = seeds;
    let mut from = String::new();
    let mut to = String::new();

    lines.next();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let line = line.trim();
        if let Some(map_name) = line.strip_suffix(" map:") {
            let split = map_name.split_once("-to-").unwrap();
            from = split.0.to_string();
            to = split.1.to_string();
        } else if !line.is_empty() {
            let mut numbers: Vec<usize> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            let interval_length = numbers.pop().unwrap();
            let source = numbers.pop().unwrap();
            let destination = numbers.pop().unwrap();
            let mapping = Mapping {
                source: source..(source + interval_length),
                destination: destination..(destination + interval_length),
            };
            result
                .mappings
                .entry((from.clone(), to.clone()))
                .or_default()
                .push(mapping);
        }
    }
    for map in result.mappings.values_mut() {
        map.sort_unstable_by(|a, b| a.source.start.cmp(&b.source.start))
    }
    result
}

fn transform(seed: usize, maps: &Maps) -> usize {
    let mappings = &maps.mappings;
    let mut result = seed;
    let mut source = String::from("seed");
    for destination in LOOKUPS {
        let destination = destination.to_string();
        let key = (source.clone(), destination.clone()); // expensive allocations
        let ranges: &Vec<Mapping> = mappings.get(&key).unwrap();
        for range in ranges {
            if range.source.contains(&result) {
                result = range.destination.start + (result - range.source.start);
                break;
            }
        }
        source = destination;
    }
    result
}

type R = Range<usize>;

fn range_intersection(a: &R, b: &R) -> R {
    if b.end <= a.start || a.end < b.start {
        0..0
    } else {
        let left = max(a.start, b.start);
        let right = min(a.end, b.end);
        left..right
    }
}

fn part1(input: &Path) -> usize {
    let mappings = read_maps(input);
    mappings
        .seeds
        .iter()
        .map(|seed| transform(*seed, &mappings))
        .min()
        .unwrap()
}

fn part2(input: &Path) -> usize {
    let mut result = usize::MAX;
    let mappings = read_maps(input);
    let mut seeds: Vec<Range<usize>> = vec![];
    let mut iter = mappings.seeds.iter();
    while let Some(start) = iter.next() {
        let len = iter.next().unwrap();
        let range = *start..start + len;
        seeds.push(range);
    }

    let mut source = String::from("seed");
    for destination in LOOKUPS {
        let new_seeds: Vec<Range<usize>> = vec![];
        let destination = destination.to_string();
        let key = (source.clone(), destination.clone());
        let ranges: &Vec<Mapping> = mappings.mappings.get(&key).unwrap();
        for seed in &seeds {
            for range in ranges {
                let intersection = range_intersection(seed, &range.source);
                if intersection.is_empty() {
                    continue;
                }
                let left = min(seed.start, intersection.end)..min(seed.end, intersection.end);
            }
        }
        source = destination;
    }
    result
}

pub fn main(_input: &Path) -> (usize, usize) {
    (part1(_input), part2(_input))
}
