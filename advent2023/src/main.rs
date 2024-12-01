use std::env::args;

use advent2023::{
    day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21,
    day22, day23, day24, day3, day4, day5, day6, day7, day8, day9,
};

fn main() {
    let argv: Vec<String> = args().collect();
    for arg in &argv[1..] {
        let day: u8 = arg.parse().unwrap_or_default();
        match day {
            1 => {
                let (p1, p2) = day1::main("input/day1.txt".into());
                println!("{p1}\t{p2}");
            }
            2 => {
                let (p1, p2) = day2::main("input/day2.txt".into());
                println!("{p1}\t{p2}");
            }
            3 => {
                let (p1, p2) = day3::main("input/day3.txt".into());
                println!("{p1}\t{p2}");
            }
            4 => {
                let (p1, p2) = day4::main("input/day4.txt".into());
                println!("{p1}\t{p2}");
            }
            5 => {
                let (p1, p2) = day5::main("input/day5.txt".into());
                println!("{p1}\t{p2}");
            }
            6 => {
                let (p1, p2) = day6::main("input/day6.txt".into());
                println!("{p1}\t{p2}");
            }
            7 => {
                let (p1, p2) = day7::main("input/day7.txt".into());
                println!("{p1}\t{p2}");
            }
            8 => {
                let (p1, p2) = day8::main("input/day8.txt".into());
                println!("{p1}\t{p2}");
            }
            9 => {
                let (p1, p2) = day9::main("input/day9.txt".into());
                println!("{p1}\t{p2}");
            }
            10 => {
                let (p1, p2) = day10::main("input/day10.txt".into());
                println!("{p1}\t{p2}");
            }
            11 => {
                let (p1, p2) = day11::main("input/day11.txt".into());
                println!("{p1}\t{p2}");
            }
            12 => {
                let (p1, p2) = day12::main("input/day12.txt".into());
                println!("{p1}\t{p2}");
            }
            13 => {
                let (p1, p2) = day13::main("input/day13.txt".into());
                println!("{p1}\t{p2}");
            }
            14 => {
                let (p1, p2) = day14::main("input/day14.txt".into());
                println!("{p1}\t{p2}");
            }
            15 => {
                let (p1, p2) = day15::main("input/day15.txt".into());
                println!("{p1}\t{p2}");
            }
            16 => {
                let (p1, p2) = day16::main("input/day16.txt".into());
                println!("{p1}\t{p2}");
            }
            17 => {
                let (p1, p2) = day17::main("input/day17.txt".into());
                println!("{p1}\t{p2}");
            }
            18 => {
                let (p1, p2) = day18::main("input/day18.txt".into());
                println!("{p1}\t{p2}");
            }
            19 => {
                let (p1, p2) = day19::main("input/day19.txt".into());
                println!("{p1}\t{p2}");
            }
            20 => {
                let (p1, p2) = day20::main("input/day20.txt".into());
                println!("{p1}\t{p2}");
            }
            21 => {
                let (p1, p2) = day21::main("input/day21.txt".into());
                println!("{p1}\t{p2}");
            }
            22 => {
                let (p1, p2) = day22::main("input/day22.txt".into());
                println!("{p1}\t{p2}");
            }
            23 => {
                let (p1, p2) = day23::main("input/day23.txt".into());
                println!("{p1}\t{p2}");
            }
            24 => {
                let (p1, p2) = day24::main("input/day24.txt".into());
                println!("{p1}\t{p2}");
            }
            _ => {
                println!("day not available");
            }
        };
    }
}
