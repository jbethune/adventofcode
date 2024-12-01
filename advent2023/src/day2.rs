use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use camino::Utf8Path as Path;

#[derive(Debug, Default)]
struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Default)]
struct Draws {
    game: usize,
    cube_draws: Vec<RGB>,
}

fn parse_line(line: &str) -> Draws {
    let mut result = Draws::default();
    if let Some((game, cubes)) = line.split_once(':') {
        result.game = if let Some((_, n)) = game.split_once(' ') {
            n.parse().unwrap()
        } else {
            panic!("could not read game number");
        };

        for round in cubes.split(';') {
            let mut rgb = RGB::default();
            for color_count in round.split(',') {
                let color_count = color_count.trim();
                if let Some((count, color_name)) = color_count.split_once(' ') {
                    let count: usize = count.parse().unwrap();
                    match color_name {
                        "red" => {
                            rgb.red = count;
                        }
                        "green" => {
                            rgb.green = count;
                        }
                        "blue" => {
                            rgb.blue = count;
                        }
                        _ => {
                            panic!("Strange color: {color_name}")
                        }
                    }
                } else {
                    panic!("Could not split count and name");
                }
            }
            result.cube_draws.push(rgb);
        }
    } else {
        panic!("Could not split line");
    }
    result
}

fn part1(input: &Path) -> usize {
    let mut result = 0;
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let game = parse_line(&line.unwrap());
        let realistic = game.cube_draws.iter().all(|round| {
            round.red <= max_red && round.green <= max_green && round.blue <= max_blue
        });
        if realistic {
            result += game.game;
        }
    }
    result
}

fn part2(input: &Path) -> usize {
    let mut result = 0;
    let reader = BufReader::new(File::open(input).unwrap());
    for line in reader.lines() {
        let game = parse_line(&line.unwrap());
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for draw in game.cube_draws {
            red = max(red, draw.red);
            green = max(green, draw.green);
            blue = max(blue, draw.blue);
        }
        let power = red * green * blue;
        result += power;
    }
    result
}

pub fn main(input: &Path) -> (usize, usize) {
    (part1(input), part2(input))
}
