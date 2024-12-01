use std::default::Default;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::take;
use std::str::FromStr;

pub enum Operand {
    Value(usize),
    Old,
}

impl Operand {
    pub fn resolve(&self, old: usize) -> usize {
        match self {
            Self::Value(v) => v.clone(),
            Self::Old => old.clone(),
        }
    }
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "old" {
            Self::Old
        } else {
            Self::Value(s.parse().unwrap())
        })
    }
}

pub enum Operator {
    Plus,
    Mul,
}

pub struct Formula {
    left: Operand,
    op: Operator,
    right: Operand,
}

impl Formula {
    pub fn apply(&self, old: usize) -> usize {
        let left = self.left.resolve(old);
        let right = self.right.resolve(old);
        match self.op {
            Operator::Plus => left + right,
            Operator::Mul => left * right,
        }
    }
}

impl Default for Formula {
    fn default() -> Self {
        let left = Operand::Old;
        let right = Operand::Old;
        let op = Operator::Plus;
        Self { left, op, right }
    }
}

#[derive(Default)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub formula: Formula,
    pub divisor: usize,
    pub to_true: usize,
    pub to_false: usize,
    pub inspections: usize,
    pub stress_tolerance: usize,
}

impl Monkey {
    pub fn throw(&mut self, common_divisor: usize) -> ThrownItems {
        let mut result = ThrownItems {
            targets: (0, 0),
            items: (vec![], vec![]),
        };
        result.targets.0 = self.to_true;
        result.targets.1 = self.to_false;

        for item in self.items.drain(..) {
            self.inspections += 1;
            let new = self.formula.apply(item % common_divisor) / self.stress_tolerance;
            if new % self.divisor == 0 {
                result.items.0.push(new);
            } else {
                result.items.1.push(new);
            }
        }
        result
    }

    pub fn catch(&mut self, items: &[usize]) {
        self.items.extend_from_slice(items);
    }
}

#[derive(Debug)]
pub struct ThrownItems {
    pub targets: (usize, usize),
    pub items: (Vec<usize>, Vec<usize>),
}

pub fn read_monkies_from_file(path: &str, reduce_stress: bool) -> Vec<Monkey> {
    let mut result = vec![];
    let mut monkey = Monkey::default();
    monkey.stress_tolerance = if reduce_stress { 3 } else { 1 };
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.trim() == "" {
            result.push(take(&mut monkey));
            monkey.stress_tolerance = if reduce_stress { 3 } else { 1 };
        } else if line.starts_with("Monkey ") {
            continue;
        } else if line.starts_with("Starting") {
            let parts: Vec<&str> = line.split(':').collect();
            let numbers: Vec<&str> = parts[1].split(',').map(|s| s.trim()).collect();
            monkey
                .items
                .extend(numbers.iter().map(|n| n.parse::<usize>().unwrap()));
        } else if line.starts_with("Operation") {
            let parts: Vec<&str> = line.split('=').collect();
            let formula_parts: Vec<&str> = parts[1].split_whitespace().collect();
            monkey.formula.left = formula_parts[0].parse().unwrap();
            monkey.formula.right = formula_parts[2].parse().unwrap();
            monkey.formula.op = if formula_parts[1] == "+" {
                Operator::Plus
            } else {
                Operator::Mul
            };
        } else if line.starts_with("Test:") {
            let parts: Vec<&str> = line.split("by").collect();
            monkey.divisor = parts[1].trim().parse().unwrap();
        } else if line.starts_with("If true") {
            let mut parts: Vec<&str> = line.split_whitespace().collect();
            monkey.to_true = parts.pop().unwrap().parse().unwrap();
        } else if line.starts_with("If false") {
            let mut parts: Vec<&str> = line.split_whitespace().collect();
            monkey.to_false = parts.pop().unwrap().parse().unwrap();
        } else {
            panic!("unrecognized line:\n{}", line);
        }
    }
    result.push(monkey);
    result
}
