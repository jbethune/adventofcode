use camino::Utf8Path as Path;

const BLANK: usize = usize::MAX;

#[derive(Debug, Clone)]
struct File {
    id: usize,
    len: u8,
    trailing_space: u8,
}

fn read_disk(input: &Path) -> Vec<File> {
    let mut result = vec![];
    let disk = std::fs::read_to_string(input).unwrap();
    let iter_file = disk.chars().step_by(2);
    let iter_space = disk.chars().skip(1).step_by(2);
    for (id, (file_len, space_len)) in iter_file.zip(iter_space).enumerate() {
        let len = file_len.to_digit(10).unwrap() as u8;
        let trailing_space = space_len.to_digit(10).unwrap_or(0) as u8;
        result.push(File {
            id,
            len,
            trailing_space,
        });
    }
    result
}

fn part1(files: &[File]) -> usize {
    let mut blocks: Vec<usize> = Vec::with_capacity(files.iter().map(|f| f.len as usize).sum());
    let mut right_file_iter = files.iter().rev();
    let mut right_file = right_file_iter.next().cloned().unwrap();
    'outer: for left_file in files {
        if left_file.id == right_file.id {
            let already_inserted = blocks.iter().filter(|b| **b == left_file.id).count();
            blocks.extend_from_slice(&vec![
                left_file.id;
                (left_file.len as usize) - already_inserted
            ]);

            break;
        }
        blocks.extend_from_slice(&vec![left_file.id; left_file.len as usize]);

        let mut space = left_file.trailing_space;
        while let Some(diff) = space.checked_sub(right_file.len) {
            space = diff;
            blocks.extend_from_slice(&vec![right_file.id; right_file.len as usize]);

            right_file = right_file_iter.next().cloned().unwrap();
            if left_file.id == right_file.id {
                // TODO handle leftover blocks. This is not an issue for my input
                break 'outer;
            }
        }

        blocks.extend_from_slice(&vec![right_file.id; space as usize]);
        right_file.len -= space;
    }
    blocks.iter().enumerate().map(|(pos, id)| pos * id).sum()
}

fn part2(files: &[File]) -> usize {
    let mut disk: Vec<usize> = Vec::with_capacity(
        files
            .iter()
            .map(|f| (f.len + f.trailing_space) as usize)
            .sum(),
    );

    let mut initial_position = Vec::with_capacity(files.len());
    for file in files {
        initial_position.push(disk.len());
        disk.extend_from_slice(&vec![file.id; file.len as usize]);
        disk.extend_from_slice(&vec![BLANK; file.trailing_space as usize]);
    }

    for (id, pos) in initial_position.iter().enumerate().rev() {
        let len = files[id].len;
        let mut streak = 0;
        let mut move_to = BLANK;
        for (i, value) in disk.iter().enumerate() {
            if i == *pos {
                break; // don't move files to the right
            }
            if *value == BLANK {
                streak += 1;
                if streak == len {
                    move_to = 1 + i - len as usize;
                    break; // found the leftmost free space
                }
            } else {
                streak = 0;
            }
        }
        if move_to != BLANK {
            // erase old values
            // and set new values
            for i in 0..len as usize {
                disk[pos + i] = BLANK;
                assert_eq!(disk[move_to + i], BLANK);
                disk[move_to + i] = id;
            }
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == BLANK { None } else { Some(i * v) })
        .sum()
}

pub fn main(input: &Path) -> (usize, usize) {
    let files = read_disk(input);
    (part1(&files), part2(&files))
}
