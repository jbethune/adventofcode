use std::collections::HashMap;

use camino::Utf8Path as Path;

fn transform(stone: usize) -> (usize, Option<usize>) {
    if stone == 0 {
        (1, None)
    } else {
        let text = stone.to_string();
        if text.len() % 2 == 0 {
            let (left, right) = text.split_at(text.len() / 2);
            (left.parse().unwrap(), Some(right.parse().unwrap()))
        } else {
            (stone * 2024, None)
        }
    }
}

fn both_parts(
    stone: usize,
    iterations: usize,
    // The `memory` parameter makes it feasible to run this for a large number of iterations,
    // because of the doubling of stones, we have almost exponential growth.
    // But because we have almost exponential growth, we also encounter the same combinations of
    // (stone_number, iterations)
    // more frequently on the same iteration level, so that it isn't too bad,
    // because we can memorize partial results
    memory: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if iterations == 0 {
        1
    } else if let Some(previous) = memory.get(&(stone, iterations)) {
        *previous
    } else {
        let result = match transform(stone) {
            (new_stone, None) => both_parts(new_stone, iterations - 1, memory),
            (new_stone, Some(extra_stone)) => {
                both_parts(new_stone, iterations - 1, memory)
                    + both_parts(extra_stone, iterations - 1, memory)
            }
        };
        memory.insert((stone, iterations), result);
        result
    }
}

pub fn main(input: &Path) -> (usize, usize) {
    let values: Vec<usize> = std::fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    // (stone_number, remaining_iterations) -> number of stones at 0 iterations remaining
    let mut memory: HashMap<(usize, usize), usize> = HashMap::new();

    (
        values
            .iter()
            .map(|stone| both_parts(*stone, 25, &mut memory))
            .sum(),
        values
            .iter()
            .map(|stone| both_parts(*stone, 75, &mut memory))
            .sum(),
    )
}
