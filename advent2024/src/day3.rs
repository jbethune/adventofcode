use camino::Utf8Path as Path;

struct Mul {
    a: usize,
    b: usize,
}

/// parse factors for a multiplication instruction (if it is valid)
fn parse_factors(mul_str: &str) -> Option<Mul> {
    let (left, right) = mul_str.split_once(",")?;
    let a: usize = left.parse().ok()?;
    let left = right.split_once(")")?.0;
    let b = left.parse().ok()?;
    Some(Mul { a, b })
}

/// gather all active multiplication instructions
fn gather_muls(parts: &[&str], out: &mut Vec<Mul>) {
    for part in parts {
        if let Some(factors) = parse_factors(part) {
            out.push(factors)
        }
    }
}

fn read_code(input: &Path, use_conditionals: bool) -> Vec<Mul> {
    let mut result = vec![];
    let data = std::fs::read_to_string(input).unwrap();
    if use_conditionals {
        let donts: Vec<&str> = data.split("don't()").collect();
        let parts: Vec<&str> = donts[0].split("mul(").collect(); // remember: implicit do() at the beginning!
        gather_muls(&parts, &mut result);

        for dont in &donts[1..] {
            if let Some((_no, yes)) = dont.split_once("do()") {
                let parts: Vec<&str> = yes.split("mul(").collect();
                gather_muls(&parts[1..], &mut result);
            }
        }
    } else {
        let parts: Vec<&str> = data.split("mul(").collect();
        gather_muls(&parts[1..], &mut result);
    }
    result
}

fn part1(input: &Path) -> usize {
    let factors = read_code(input, false);
    factors.iter().map(|f| f.a * f.b).sum()
}

fn part2(input: &Path) -> usize {
    let factors = read_code(input, true);
    factors.iter().map(|f| f.a * f.b).sum()
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
