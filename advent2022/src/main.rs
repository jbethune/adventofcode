use std::env::args;

use chrono::offset::Utc;
use chrono::Datelike;

use advent2022::dispatch;

fn main() {
    let argv: Vec<String> = args().collect();
    let day: usize = if let Some(day) = argv.get(1) {
        day.parse().unwrap()
    } else {
        Utc::now().day() as usize
    };
    if let Some((part1, part2)) = dispatch(day) {
        println!("The solutions are part1={} part2={}", part1, part2);
    } else {
        eprintln!("The day is not available yet");
    }
}
