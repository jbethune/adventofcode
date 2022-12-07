use std::collections::HashSet;
use std::io::{BufRead, BufReader};

pub struct Rucksack {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Rucksack {
    pub fn new(line: String) -> Self {
        let mut left = vec![];
        let mut right = vec![];
        let num_items: usize = line.len() / 2;
        for (i, c) in line.chars().enumerate() {
            let value = get_value(c);
            if i < num_items {
                left.push(value);
            } else {
                right.push(value);
            }
        }
        Self { left, right }
    }

    pub(crate) fn get_duplicate_item(&self) -> usize {
        let mut set_left = HashSet::new();
        for item in self.left.iter() {
            set_left.insert(item.clone());
        }
        let mut set_right = HashSet::new();
        for item in self.right.iter() {
            set_right.insert(item.clone());
        }
        for duplicate in set_left.intersection(&set_right) {
            return *duplicate;
        }
        0
    }

    pub(crate) fn to_set(&self) -> HashSet<usize> {
        let mut result = HashSet::from_iter(self.left.iter().map(|x| x.clone()));
        result.extend(self.right.iter());
        result
    }
}

fn get_value(c: char) -> usize {
    let numeric = c as usize;
    if 'A' <= c && c <= 'Z' {
        numeric - 65 + 27
    } else {
        numeric - 97 + 1
    }
}

pub fn get_rucksacks_from_file(path: &str) -> Vec<Rucksack> {
    let mut result = vec![];
    let fd = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let rucksack = Rucksack::new(line.unwrap());
        result.push(rucksack);
    }
    result
}

pub fn get_badge(r1: &Rucksack, r2: &Rucksack, r3: &Rucksack) -> usize {
    let s1 = r1.to_set();
    let s2 = r2.to_set();
    let s3 = r3.to_set();
    let set4: HashSet<usize> = HashSet::from_iter(s1.intersection(&s2).into_iter().cloned());
    for badge in set4.intersection(&s3) {
        return *badge;
    }
    0
}
