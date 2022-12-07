use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn sorted_calories_from_file(path: &str) -> Vec<usize> {
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    let mut tallies = vec![0];
    let mut index = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim() == "" {
            index += 1;
            tallies.push(0);
        } else {
            tallies[index] += line.parse::<usize>().unwrap();
        }
    }
    tallies.sort();
    tallies
}
