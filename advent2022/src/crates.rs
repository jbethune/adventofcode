use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Crate = char;

pub struct Warehouse {
    stacks: Vec<Vec<Crate>>,
}

impl Warehouse {
    pub(crate) fn apply(&mut self, instruction: &MoveInstruction, all_at_once: bool) {
        if all_at_once {
            let source = &mut self.stacks[instruction.from];
            let intermediate = source.split_off(source.len() - instruction.amount);
            self.stacks[instruction.to].extend(intermediate);
        } else {
            for _ in 0..instruction.amount {
                let value = self.stacks[instruction.from].pop().unwrap();
                self.stacks[instruction.to].push(value);
            }
        }
    }

    pub(crate) fn report_top_row(&self) {
        let mut result = String::with_capacity(10);
        for stack in &self.stacks {
            result.push(*stack.last().unwrap())
        }
        println!("top row: {}", result);
    }
}

pub struct MoveInstruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

pub fn read_stacks_from_file(path: &str) -> Warehouse {
    let mut stacks = vec![];
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim_end();
        if line.starts_with(" 1 ") {
            break;
        }

        let mut iter = line.chars();
        for i in 0..10 {
            if let Some(left_brace) = iter.next() {
                if left_brace != '[' {
                    iter.next(); // iter.skip() takes ownership and that causes issues
                    iter.next();
                    iter.next();
                    continue;
                }
            } else {
                break;
            }
            let name = &iter.next().unwrap();
            stacks[i].push(*name);
            let right_brace = &iter.next().unwrap();
            assert_eq!(*right_brace, ']');
            if let Some(space) = &iter.next() {
                assert_eq!(*space, ' ');
            } // else: EOL
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    Warehouse { stacks }
}

pub fn read_instructions_from_file(path: &str) -> Vec<MoveInstruction> {
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    let mut search = true;
    let mut result = Vec::new();
    for line in reader.lines() {
        if search {
            if line.unwrap().trim() == "" {
                search = false
            }
        } else {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(' ').collect();
            let amount = parts[1].parse().unwrap();
            let from = parts[3].parse::<usize>().unwrap() - 1;
            let to = parts[5].parse::<usize>().unwrap() - 1;
            result.push(MoveInstruction { amount, from, to });
        }
    }
    result
}
