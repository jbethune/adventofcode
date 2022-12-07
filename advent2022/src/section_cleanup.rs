use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

#[derive(Debug)]
pub struct Assignment {
    a: Range<usize>,
    b: Range<usize>,
}

impl Assignment {
    pub fn is_completely_contained(&self) -> bool {
        let a = &self.a;
        let b = &self.b;
        if a.start < b.start {
            a.end >= b.end
        } else if b.start < a.start {
            b.end >= a.end
        } else {
            true // equal
        }
    }

    pub fn overlaps(&self) -> bool {
        // not the most efficient way, but it works:
        self.a.contains(&self.b.start) || self.b.contains(&self.a.start)
    }
}

pub fn get_assignments_from_file(path: &str) -> Vec<Assignment> {
    let mut result = vec![];
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(',').collect();
        assert_eq!(split.len(), 2);
        let left: Vec<&str> = split[0].split('-').collect();
        let right: Vec<&str> = split[1].split('-').collect();
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        let a = left[0].parse().unwrap()..(left[1].parse::<usize>().unwrap() + 1);
        let b = right[0].parse().unwrap()..(right[1].parse::<usize>().unwrap() + 1);
        result.push(Assignment { a, b });
    }
    result
}
