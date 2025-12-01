use std::env::args;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
                let (p1, p2) = day12::main("input/day12b.txt".into());
                println!("{p1}\t{p2}");
            }
            _ => {
                println!("day not available");
            }
        };
    }
}
