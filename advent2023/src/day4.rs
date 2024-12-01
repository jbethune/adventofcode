use std::{
    cmp::Ordering,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

#[derive(Debug)]
struct Card {
    hits: usize,
}

impl Card {
    fn new(winning: Vec<usize>, given: Vec<usize>) -> Self {
        let mut hits = 0;
        let mut givens = given.iter().peekable();
        // sure, I could just have used a HashSet...
        // but this is more fun :-)
        for winner in &winning {
            while let Some(&given) = givens.peek() {
                match given.cmp(winner) {
                    Ordering::Less => {
                        givens.next();
                    }
                    Ordering::Equal => {
                        hits += 1;
                        givens.next();
                    }
                    Ordering::Greater => break,
                }
            }
        }

        Self { hits }
    }
}

fn points(matches: usize) -> usize {
    match matches {
        0 => 0,
        1 => 1,
        n => 1 << (n - 1),
    }
}

fn extract_numbers(line: &str) -> Card {
    let (_, rest) = line.split_once(": ").unwrap();
    let (lhs, rhs) = rest.split_once(" | ").unwrap();
    let lhs = lhs.split(' ');
    let rhs = rhs.split(' ');
    let mut winning: Vec<usize> = lhs.filter_map(|n| n.parse().ok()).collect();
    let mut given: Vec<usize> = rhs.filter_map(|n| n.parse().ok()).collect();
    winning.sort_unstable();
    given.sort_unstable();
    Card::new(winning, given)
}

fn part1(input: &Path) -> usize {
    let mut result = 0;
    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let numbers = extract_numbers(&line.unwrap());
        result += points(numbers.hits);
    }
    result
}

fn part2(input: &Path) -> usize {
    let mut result = 0;
    let num_lines = std::fs::read_to_string(input)
        .unwrap()
        .matches('\n')
        .count()
        + 1;

    let cards = {
        let mut cards = Vec::with_capacity(num_lines + 1);
        cards.push(Card::new(vec![], vec![])); // dummy index 0
        let reader = BufReader::new(File::open(input).unwrap());
        for line in reader.lines() {
            cards.push(extract_numbers(&line.unwrap()));
        }
        cards
    };
    let mut queue = VecDeque::with_capacity(2 * cards.len());
    for i in 1..cards.len() {
        // we automatically win all originals
        queue.push_back(i);
    }
    while let Some(card_index) = queue.pop_front() {
        result += 1;
        let card = &cards[card_index];
        for i in (card_index + 1)..(card_index + card.hits + 1) {
            queue.push_back(i);
        }
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
