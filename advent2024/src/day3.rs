use camino::Utf8Path as Path;

/// parse factors and multiply them if the instruction is valid
fn parse_factors(mul_str: &str) -> Option<usize> {
    let (left, right) = mul_str.split_once(",")?;
    let a: usize = left.parse().ok()?;
    let left = right.split_once(")")?.0;
    let b: usize = left.parse().ok()?;
    Some(a * b)
}

/// gather and execute all active multiplication instructions
fn gather_muls(parts: &[&str], out: &mut Vec<usize>) {
    for part in parts {
        if let Some(factors) = parse_factors(part) {
            out.push(factors)
        }
    }
}

fn read_code(input: &Path, use_conditionals: bool) -> Vec<usize> {
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
    read_code(input, false).iter().sum()
}

fn part2(input: &Path) -> usize {
    read_code(input, true).iter().sum()
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
