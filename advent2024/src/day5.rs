use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

fn read_data(input: &Path) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut forbidden_orderings = HashSet::new();
    let mut all_pages = vec![];
    let reader = BufReader::new(File::open(input).unwrap());
    let mut read_ordering = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if read_ordering {
            if let Some((a, b)) = line.split_once("|") {
                forbidden_orderings.insert((b.parse().unwrap(), a.parse().unwrap()));
            } else {
                assert_eq!(line, "");
                read_ordering = false;
            }
        } else {
            let pages: Vec<usize> = line
                .split(',')
                .map(|page| page.parse::<usize>().unwrap())
                .collect();
            all_pages.push(pages);
        }
    }
    (forbidden_orderings, all_pages)
}

fn part1(input: &Path) -> usize {
    let mut result = 0;
    let (forbidden, all_pages) = read_data(input);
    for pageset in all_pages {
        let mut found_error = false;
        for pair in pageset.windows(2) {
            let pair = (pair[0], pair[1]);
            if forbidden.contains(&pair) {
                found_error = true;
                break;
            }
        }
        if !found_error {
            result += pageset[pageset.len() / 2];
        }
    }
    result
}

fn part2(input: &Path) -> usize {
    let (forbidden, mut all_pages) = read_data(input);
    let mut result = 0;
    for pageset in &mut all_pages {
        let mut faulty_pageset = false;
        let mut seen_error = true; // initial value to get loop started
        while seen_error {
            seen_error = false;
            for (i, win) in pageset.windows(2).enumerate() {
                if forbidden.contains(&(win[0], win[1])) {
                    pageset.swap(i, i + 1);
                    seen_error = true;
                    faulty_pageset = true;
                    break; // start from beginning, tolerating quadratic runtime
                }
            }
        }
        if faulty_pageset {
            result += pageset[pageset.len() / 2];
        }
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
