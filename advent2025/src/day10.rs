use camino::Utf8Path as Path;

fn read_input(path: &Path) -> String {
    path.file_name().unwrap().to_string()
}

fn part1(data: &str) -> usize {
    data.len()
}

fn part2(data: &str) -> usize {
    data.len()
}

pub fn main(input: &Path) -> (usize, usize) {
    let data = read_input(input);
    (part1(&data), part2(&data))
}
