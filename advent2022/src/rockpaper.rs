use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Weapon {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn answer(&self, weapon: &Weapon) -> Weapon {
        match self {
            Outcome::Win => {
                match weapon {
                    Weapon::Rock => Weapon::Paper,
                    Weapon::Paper => Weapon::Scissors,
                    Weapon::Scissors => Weapon::Rock,
                }
            },
            Outcome::Lose => {
                match weapon {
                    Weapon::Rock => Weapon::Scissors,
                    Weapon::Paper => Weapon::Rock,
                    Weapon::Scissors => Weapon::Paper,
                }
            },
            Outcome::Draw => {
                weapon.clone()
            }
        }
    }
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Bad data"),
        }
    }
}

impl Weapon {
    fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl From<char> for Weapon {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Something else")
        }
    }
}

pub struct Choice {
    you: Weapon,
    me: Weapon,
}

impl Choice {
    pub fn new(you: Weapon, me: Weapon) -> Self {
        Self{you, me}
    }

    pub fn score(&self) -> usize {
        let base = if self.you == self.me {
            3
        } else {
            match self.you {
                Weapon::Rock => {
                    if self.me == Weapon::Paper {
                        6
                    } else {
                        0
                    }
                },
                Weapon::Paper => {
                    if self.me == Weapon::Scissors {
                        6
                    } else {
                        0
                    }
                },
                Weapon::Scissors => {
                    if self.me == Weapon::Rock {
                        6
                    } else {
                        0
                    }
                },
            }
        };
        base + self.me.value()
    }
}

pub fn read_choices_from_file(path: &str) -> Vec<Choice> {
   let mut result = vec![];
   let fd = File::open(path).unwrap();
   let reader = BufReader::new(fd);
   for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.chars();
        let you: Weapon = iter.next().unwrap().into();
        iter.next();
        let me: Weapon = iter.next().unwrap().into();
        result.push(Choice::new(you, me));
   }
   result
}

pub fn read_outcome_based_choices_from_file(path: &str) -> Vec<Choice> {
   let mut result = vec![];
   let fd = File::open(path).unwrap();
   let reader = BufReader::new(fd);
   for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.chars();
        let you: Weapon = iter.next().unwrap().into();
        iter.next();
        let outcome: Outcome = iter.next().unwrap().into();
        result.push(Choice::new(you, outcome.answer(&you)));
   }
   result
}
