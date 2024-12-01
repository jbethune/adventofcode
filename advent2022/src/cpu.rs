use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Noop,
    Addx(isize),
}

pub struct CPU {
    cycle: usize,
    x: isize,
    queue: Vec<Command>,
    framebuffer: Vec<bool>,
}

impl CPU {
    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            cycle: 0,
            x: 1,
            queue: commands,
            framebuffer: Vec::with_capacity(40 * 6),
        }
    }

    pub fn run(&mut self, checkpoints: &[usize]) -> Vec<isize> {
        let mut result = vec![];
        let mut iter = checkpoints.iter();
        let mut checkpoint = iter.next().cloned().unwrap();
        for command in &self.queue {
            let cycles = match command {
                Command::Noop => 1,
                Command::Addx(_) => 2,
            };
            for _ in 0..cycles {
                self.cycle += 1;
                self.framebuffer.push(
                    self.x.is_positive()
                        && ((self.x as usize) % 40).abs_diff((self.cycle - 1) % 40) <= 1,
                );

                if self.cycle == checkpoint {
                    result.push(self.cycle as isize * self.x);
                    if let Some(v) = iter.next() {
                        checkpoint = *v;
                    } else {
                        checkpoint = 0; // impossible
                    }
                }
            }
            if let Command::Addx(v) = command {
                self.x += v;
            }
        }
        println!("final cycle: {}", self.cycle);
        result
    }

    pub fn render_screen(&self) {
        for (i, v) in self.framebuffer.iter().enumerate() {
            print!("{}", if *v { '#' } else { '.' });
            if (i + 1) % 40 == 0 {
                println!("");
            }
        }
    }
}

pub fn read_commands_from_file(path: &str) -> Vec<Command> {
    let mut result = vec![];
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.trim().split(' ').collect();
        result.push(match parts[0] {
            "noop" => Command::Noop,
            "addx" => Command::Addx(parts[1].parse().unwrap()),
            _ => panic!("{}", parts[0]),
        });
    }
    result
}
